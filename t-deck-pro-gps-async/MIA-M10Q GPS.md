The MIA-M10Q module uses the readable NMEA protocol for data output but requires the proprietary binary UBX protocol for configuration. All configuration changes, such as altering power modes or message rates, are performed by sending UBX messages to the device. The following steps detail how to initialize, restart, configure, and read from the device using a serial connection.

### **Command Preamble: Understanding UBX Messages**

The commands shown below are in hexadecimal format. They follow the u-blox UBX protocol structure. To ensure the configuration is saved through power cycles, the examples target both RAM and the Battery-Backed RAM (BBR). Each UBX command has the following structure:

* **Header**: `B5 62`  
* **Class/ID**: Identifies the message type (e.g., `06 8A` for configuration setting).  
* **Length**: Length of the payload in bytes (little-endian).  
* **Payload**: The message-specific data.  
* **Checksum**: Two bytes calculated over the Class, ID, Length, and Payload fields.

---

### **1\. Initialization**

Initialization involves connecting the hardware and establishing serial communication. The module starts automatically upon receiving power with its default settings.

* **Step 1: Hardware Connection**  
  * Connect the module's  
     `VCC` pin to your power supply (e.g., 3.3V) and the `V_IO` pin to your logic level voltage (e.g., 3.3V). For a simple 3.3V design, these can be connected together.  
  * Connect the module's UART `TX` (pin G1) to your microcontroller's `RX` pin.  
  * Connect the module's UART `RX` (pin H1) to your microcontroller's `TX` pin.  
  * Connect all `GND` pins to your system ground.  
  * Leave the  
     `RESET_N` pin open for normal operation.  
* **Step 2: Establish Serial Communication**  
  * Configure your microcontroller's UART interface to the module's default settings:  
     **38400 baud, 8 data bits, no parity, 1 stop bit**.  
  * The module will begin outputting default NMEA messages immediately upon startup.  
* **Step 3: Verify Communication (Optional but Recommended)**  
  * Send a `UBX-MON-VER` poll request to the module. This asks the device to report its software and hardware version, confirming that two-way communication is working.  
  * **Command:** `B5 62 0A 04 00 00 0E 34`  
  * The module will respond with a `UBX-MON-VER` message.

---

### **2\. Restarting the Device**

The module can be restarted via a hardware pin or software commands. Different restart types determine what information is cleared from memory.

* **Hardware Reset (Cold Start)**  
  * Driving the  
     **`RESET_N`** pin low for at least 1 ms will trigger a hardware reset. This is a "cold start" that clears all configuration, orbit data, and the real-time clock from both RAM and BBR. Use this only in critical situations.  
* **Software Reset (Using UBX-CFG-RST)**  
  * You can send a  
     `UBX-CFG-RST` message to trigger specific types of restarts without a hardware pin.  
  * **Cold Start:** Clears all ephemeris, almanac, time, and position data from BBR. The receiver must search for all satellites from scratch.  
    * **Command:** `B5 62 06 04 04 00 FF FF 00 00 08 15`  
  * **Warm Start:** Clears ephemeris data but preserves almanac, position, and time. This is typical if the device has been off for more than four hours.  
    * **Command:** `B5 62 06 04 04 00 01 00 01 00 10 19`  
  * **Hot Start:** Preserves all data for the fastest possible startup (typically when the device has been off for less than four hours).  
    * **Command:** `B5 62 06 04 04 00 00 00 01 00 0F 17`

---

### **3\. Configuration (Power Saving & Message Filtering)**

To save battery and reduce the processing load on your main controller, you can adjust the update rate, enable power-save modes, and disable unneeded NMEA messages. These configurations are done using `UBX-CFG-VALSET` messages.

* **Reducing Unnecessary Messages**  
  * You can reduce serial traffic by disabling NMEA messages you don't need. The message output rate is configured individually for each message and communication port. A rate of  
     `0` disables the message, while a rate of `1` sends it with every navigation solution.  
  * **Example: Disable GLL, GSA, GSV, and VTG messages on UART1.** This leaves the common GGA and RMC messages enabled.  
    * **Command:** `B5 62 06 8A 1D 00 00 03 00 00 84 00 91 10 00 75 00 91 10 00 6B 00 91 10 00 61 00 91 10 00 F6 16`  
* **Changing the Update Rate**  
  * The simplest way to save power is to calculate a position fix less frequently. This is controlled by the measurement rate (`CFG-RATE-MEAS`). For example, setting this to 5000 ms will produce one fix every 5 seconds.  
  * **Example: Set the navigation update rate to every 5 seconds (5000 ms).**  
    * **Command:** `B5 62 06 8A 0A 00 00 03 00 00 01 00 21 30 88 13 58 06`  
* **Enabling Power Save Mode (PSM)**  
  * For significant power savings, especially for asset tracking applications, you can use Power Save Mode (PSM). This mode switches the receiver on and off to save power. The  
     **PSMOO (On/Off)** mode is ideal for update periods longer than 10 seconds.  
  * **Important:** When using PSM, it's recommended to disable SBAS, as the receiver cannot process its data in this mode. Also, BeiDou B1C signals are not supported in PSM; the receiver uses BeiDou B1I instead. An RTC is required for PSMOO operation.  
  * **Example: Enable PSMOO with a 60-second update period.** The receiver will wake up, attempt a fix for a short duration (`onTime`), and then sleep until the next 60-second interval.  
    * **Command:** `B5 62 06 8A 11 00 00 03 00 00 01 00 40 20 01 03 00 40 40 60 EA 00 00 01 16`  
    * *(This single command sets `CFG-PM-OPERATEMODE` to PSMOO and `CFG-PM-POSUPDATEPERIOD` to 60,000 ms)*.

---

### **4\. Reading the Device**

Once configured, reading data is a matter of parsing the NMEA messages sent by the module over the serial connection.

* **Step 1: Read the Serial Port**  
  * Continuously read the incoming data from your controller's UART receive buffer. NMEA messages are ASCII text, line-based (ending with `\r\n`), and start with a `$` character.  
* **Step 2: Parse NMEA Messages**  
  * Your firmware should parse the standard NMEA sentences you have enabled (e.g., `$GNRMC`, `$GNGGA`). These sentences contain position, velocity, time, and fix status.  
* **Step 3: Check Data Validity**  
  * It is critical to check the "valid" flag within the NMEA messages before using the data.  
  * In the **RMC** sentence, this is the second field ('V' \= Void, 'A' \= Active/Valid).  
  * In the **GGA** sentence, this is the sixth field ('0' \= Invalid, '1' \= GPS fix, '2' \= DGPS fix, etc.).  
  * Do not use data from any message marked as invalid.


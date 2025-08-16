### **How to Use These Commands**

The following examples use the **`UBX-CFG-VALSET`** message to configure the receiver. This message allows you to set multiple configuration keys in a single command. The configuration is set for both

**RAM** and **BBR** (Battery-Backed RAM) to ensure the settings persist after a reboot or power cycle.

You will need to send these hexadecimal byte sequences to the GPS module via its UART or I2C interface.

---

### **Scenario 1: Normal Mode (1Hz Updates)**

This configuration sets the receiver to a standard 1Hz navigation rate and outputs the most common position messages (`NMEA-GGA`, `NMEA-RMC`, and `UBX-NAV-PVT`) once per second. Static hold is disabled.

#### **Strategy**

* Set the measurement rate (  
  `CFG-RATE-MEAS`) to 1000 ms for a 1Hz update.  
* Set the navigation rate (  
  `CFG-RATE-NAV`) to 1, meaning a navigation solution is calculated for every measurement.  
* Enable  
   `NMEA-GGA`, `NMEA-RMC`, and `UBX-NAV-PVT` messages for the UART1 interface with a rate of 1 (output every navigation epoch).  
* Set the dynamic platform model to "Portable," which is suitable for most general-purpose applications.

#### **UBX Command**

This single `UBX-CFG-VALSET` message configures all necessary keys.

B5 62 06 8A 24 00 01 07 00 00 01 00 21 30 E8 03 02 00 21 30 01 21 00 11 20 00 A8 00 91 20 01 B1 00 91 20 01 07 00 91 20 01 34 16

**Breakdown of the command's configuration keys:**

* `CFG-RATE-MEAS` \= 1000 (0x03E8)  
* `CFG-RATE-NAV` \= 1  
* `CFG-NAVSPG-DYNMODEL` \= 0 (Portable)  
* `CFG-MSGOUT-NMEA_ID_GGA_UART1` \= 1  
* `CFG-MSGOUT-NMEA_ID_RMC_UART1` \= 1  
* `CFG-MSGOUT-UBX_NAV_PVT_UART1` \= 1

---

### **Scenario 2: Update Only on Movement**

This configuration minimizes communication by only sending location updates when speed is greater than 0.2 m/s and the position has changed by more than 2 meters.

#### **Strategy**

This uses the receiver's **Static Hold** feature. When the receiver's speed drops below a threshold, it freezes its position output until a significant displacement is detected.

* **Receiver-Side:**  
  * Set  
     `CFG-MOT-GNSSSPEED_THRS` to 20 (cm/s) to activate static hold below 0.2 m/s.  
  * Set  
     `CFG-MOT-GNSSDIST_THRS` to 2 (meters) to exit static hold only after moving 2 meters from the held position.  
  * Disable all periodic messages except `UBX-NAV-PVT`, which contains all the necessary flags and data for the host.  
* **Host-Side Logic:**  
  * The receiver will **continue to send** `UBX-NAV-PVT` messages at 1Hz.  
  * Your host application must inspect the `UBX-NAV-PVT` message. When the receiver is in static hold, the `gSpeed` (ground speed) field will be 0, and the position will not change.  
  * Your application should **ignore** these messages and only process them when the speed is non-zero, indicating movement has resumed.

#### **UBX Command**

B5 62 06 8A 18 00 01 07 00 00 24 00 11 20 14 25 00 11 20 02 07 00 91 20 01 2F C9

**Breakdown of the command's configuration keys:**

* `CFG-MOT-GNSSSPEED_THRS` \= 20  
* `CFG-MOT-GNSSDIST_THRS` \= 2  
* `CFG-MSGOUT-UBX_NAV_PVT_UART1` \= 1  
* *(This command implicitly relies on other messages being disabled. You may need to send additional commands to turn off NMEA messages if they are enabled by default)*.

---

### **Scenario 3: Update Every Minute, if Moved**

This configuration provides a location fix every minute, but only if the device has moved more than 5 meters since the last reported fix.

#### **Strategy**

This requires a combination of receiver configuration and host logic, as the receiver cannot natively combine a time condition with a distance condition for message output.

* **Receiver-Side:**  
  1. Configure  
      **Static Hold** with a distance threshold of 5 meters (`CFG-MOT-GNSSDIST_THRS` \= 5). This makes the receiver aware of significant movement.  
  2. Keep the navigation rate at 1Hz to allow the receiver to quickly detect when it has exited the static hold state.  
  3. Enable only the `UBX-NAV-PVT` message.  
* **Host-Side Logic:**  
  1. Your application will receive `UBX-NAV-PVT` messages at 1Hz from the receiver.  
  2. Maintain two variables in your code: `lastReportedPosition` and `lastReportedTime`.  
  3. When you receive a message, check if the receiver is out of static hold (i.e., speed is \> 0).  
  4. If it is out of static hold AND `(currentTime - lastReportedTime > 60 seconds)`, then:  
     * Process the new location.  
     * Update `lastReportedPosition` with the new coordinates.  
     * Update `lastReportedTime` to the current time.  
  5. If either condition is not met, simply ignore the incoming message.

#### **UBX Command**

B5 62 06 8A 18 00 01 07 00 00 24 00 11 20 1E 25 00 11 20 05 07 00 91 20 01 13 8C

**Breakdown of the command's configuration keys:**

* `CFG-MOT-GNSSSPEED_THRS` \= 30 (default low speed)  
* `CFG-MOT-GNSSDIST_THRS` \= 5  
* `CFG-MSGOUT-UBX_NAV_PVT_UART1` \= 1

### **Command to Turn Off the GPS (Software Standby)**

To turn off the GPS using a software command, you can place it into **Software Standby Mode**. In this mode, the main power (

`VCC`) is disabled internally, and the module enters a very low-power state. It can be woken up by activity on a communication port (like UART RX) or other configured wakeup sources.

**Important:** Entering software standby clears the receiver's RAM. Any configuration you want to keep must be saved to a non-volatile layer like BBR (Battery-Backed RAM) beforehand.

#### **UBX Command (UBX-RXM-PMREQ)**

Send the following hexadecimal command to the module to enter software standby mode immediately.

B5 62 02 41 08 00 00 00 00 00 02 00 00 00 4D 3B

**Breakdown of the command:**

* This is a `UBX-RXM-PMREQ` message.  
* `duration` is set to `0`, meaning it will remain in standby until a wakeup event occurs.  
* `flags` are set to `0x00000002`, which sets the `backup` flag. The receiver will enter the low-power backup state.  
* The `force` flag must also be used, which is handled by sending this message while the receiver is running.

---

### **Power-Saving Mode: Update Only When Location Changes**

This mode combines two features: **Power Save Mode (PSMOO)** to save power by waking up periodically, and **Static Hold** to detect movement.

#### **Strategy**

1. **Receiver Configuration:**  
   * The receiver is put into  
      **Power Save Mode On/Off (PSMOO)**, where it wakes up at a defined interval (e.g., every 60 seconds), gets a position fix, and then goes back to sleep. This dramatically reduces average power consumption.  
   * The **Static Hold** feature is enabled. If the receiver wakes up and determines its position has not moved by a defined distance (e.g., 5 meters), it will report a velocity of zero.  
2. **Host Processor Logic (Required):**  
   * Your main application code must read the messages from the GPS.  
   * When a location message is received, check the **speed** field.  
   * If the **speed is greater than zero**, the device has moved. You should process and use this new location.  
   * If the **speed is zero**, the device is in the "Static Hold" state, meaning it has not moved significantly since the last wake-up. You should **ignore** this position update.

#### **UBX Command (UBX-CFG-VALSET)**

This single command configures the receiver for a 60-second wake-up interval and a 5-meter movement threshold. All settings are saved to BBR to persist after sleeping.

B5 62 06 8A 20 00 01 07 00 00 01 00 41 20 02 02 00 41 40 60 EA 00 00 24 00 11 20 1E 25 00 11 20 05 07 00 91 20 01 03 61

**Breakdown of the command's configuration keys:**

* `CFG-PM-OPERATEMODE` \= `2` (PSMOO \- Power Save Mode On/Off)  
* `CFG-PM-POSUPDATEPERIOD` \= `60000` (Wake up every 60,000 ms)  
* `CFG-MOT-GNSSSPEED_THRS` \= `30` (30 cm/s, a reasonable default speed threshold)  
* `CFG-MOT-GNSSDIST_THRS` \= `5` (5-meter distance threshold to exit static hold)  
* `CFG-MSGOUT-UBX_NAV_PVT_UART1` \= `1` (Ensures the main position/velocity/time message is enabled)


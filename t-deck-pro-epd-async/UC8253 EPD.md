This is the communication sequence for initializing the UC8253, performing an initial fast full-screen update, and then performing subsequent fast partial updates for a 2-color display.

---

### **I. Initialization and First Full Update**

This sequence configures the driver and performs the first screen write.

1. **Hardware Reset:**  
   * Apply power to VDD, VDDA, and VDDIO.  
   * Pull the `RST_N` pin low for at least 50µs and then release it to reset the driver. \[160\] Wait for at least 1ms before sending the first command. \[1138\]  
2. **Initial Configuration:** Send the following commands to configure the driver for a fast, 2-color (KW) mode update on a 240x320 panel:  
   * **`Power Setting (PWR)`** (R01H): Enable internal DC/DC functions and set the required VGH/VGL and VSH/VSL voltage levels for your panel. \[340, 344, p. 16\]  
   * **`Booster Soft Start (BTST)`** (R06H): Configure booster timing and driving strength. \[p. 19\]  
   * **`PLL control (PLL)`** (R30H): Set a high internal frame rate (e.g., 100Hz or higher) using the `FRS` bits to accelerate LUT execution. \[615, 623\]  
   * **`TCON setting (TCON)`** (R60H): Set the `S2G` and `G2S` non-overlap periods to a minimal value to reduce line-scan overhead. \[778\]  
   * **`VCOM and data interval setting (CDI)`** (R50H): Configure the VCOM and data output interval. \[695\]  
   * **`Resolution setting (TRES)`** (R61H): Set the panel resolution to 240x320 by writing to the `HRES[7:3]` and `VRES[8:0]` registers. \[796, 801\]  
   * **`Panel Setting (PSR)`** (R00H): Set `KW/R=1` to enable 2-color (KW) mode \[261\] and `REG=1` to use LUTs loaded into the registers. \[255\]  
3. **Load Look-Up Tables (LUTs):**  
   * To achieve a fast update, send optimized (short waveform) LUTs to the driver.  
   * Send the **`LUTWW`** (R21H), **`LUTKW`** (R22H), **`LUTWK`** (R23H), and **`LUTKK`** (R24H) commands, each followed by its 43-byte data payload defining the waveform for that pixel transition. \[pp. 23-24\]  
4. **Perform Full-Screen Update:**  
   * Send the **`Power ON (PON)`** command (R04H) and wait for the `BUSY_N` pin to go HIGH, indicating that the power rails are stable. \[391, 393\]  
   * Send **`DATA START TRANSMISSION 1 (DTM1)`** (R10H) followed by the entire "old" image data buffer (240x320 pixels). For a first-time update, this might be an all-white frame. \[420\]  
   * Send **`DATA START TRANSMISSION 2 (DTM2)`** (R13H) followed by the entire "new" target image data buffer. \[449\]  
   * Send the **`Display Refresh (DRF)`** command (R12H). The `BUSY_N` pin will go LOW while the panel updates. \[434, 435\]  
   * Wait for the `BUSY_N` pin to return HIGH, signaling the end of the update.  
   * Send the **`Power OFF (POF)`** command (R02H) to conserve power. \[374\]

---

### **II. Subsequent Partial Updates**

This sequence updates only a small, defined portion of the screen.

1. **Enter Partial Mode:**  
   * Send the **`Partial Window (PTL)`** command (R90H) with the horizontal and vertical start/end coordinates (`HRST`, `HRED`, `VRST`, `VRED`) that define the update region. \[930\]  
   * Send the **`Partial In (PTIN)`** command (R91H) to switch the driver to partial update mode. \[945\]  
2. **Perform Partial Update:**  
   * Send the **`Power ON (PON)`** command (R04H) and wait for `BUSY_N` to go HIGH. \[393\]  
   * Send the **`DTM1`** command (R10H) followed by only the "old" image data for the defined partial window.  
   * Send the **`DTM2`** command (R13H) followed by only the "new" image data for the defined partial window.  
   * Send the **`Display Refresh (DRF)`** command (R12H). The driver will update only the specified window. \[434\]  
   * Wait for `BUSY_N` to return HIGH.  
   * Send the **`Power OFF (POF)`** command (R02H).  
3. **Exit Partial Mode (Optional):**  
   * To return to full-screen updates, send the **`Partial Out (PTOUT)`** command (R92H). \[949\]


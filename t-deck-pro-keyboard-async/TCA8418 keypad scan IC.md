Here is a specification for communicating with the TCA8418 keypad scan IC based on the provided datasheet and your hardware configuration.

The I2C slave address for the device is

**0x34**, which matches your defined

`BOARD_I2C_ADDR_KEYBOARD`. The communication protocol is standard I2C, using the SCL and SDA lines. The

`INT` pin is an **active-low, open-drain output**, which should be connected to an interrupt-capable input pin on your processor, configured to trigger on a **falling edge**.

---

### **\#\# Initialization Sequence**

After a power-on reset, the following I2C write transactions are required to configure the TCA8418 for a **4-row by 10-column QWERTY keypad**.

**1\. Configure Keypad Matrix Pins** You must tell the IC which of its GPIOs are connected to your keypad matrix. A '1' in a bit position assigns that pin to the keypad matrix.

* **Write `0x0F` to Register `0x1D` (KP\_GPIO1):** This configures **ROW0 through ROW3** for the keypad matrix (`0b00001111`).  
* **Write `0xFF` to Register `0x1E` (KP\_GPIO2):** This configures **COL0 through COL7** for the keypad matrix (`0b11111111`).  
* **Write `0x03` to Register `0x1F` (KP\_GPIO3):** This configures **COL8 and COL9** for the keypad matrix (`0b00000011`).

**2\. Configure Operating Mode and Interrupts** This step enables the necessary interrupts and sets the operating parameters in the main configuration register.

* **Write `0x09` to Register `0x01` (CFG):** This sets the following configuration (`0b00001001`):  
  * **KE\_IEN (Bit 0\) \= 1:** Enables the Key Event Interrupt. This is the primary interrupt for key presses and releases.  
  * **OVR\_FLOW\_IEN (Bit 3\) \= 1:** Enables the FIFO overflow interrupt. This is good practice to ensure no key events are silently lost.  
  * Other bits are left at their default '0' state, which disables features like auto-increment, keypad lock, and GPI events.

**3\. Clear Interrupt Status** It is recommended to clear any stale interrupts that may be present after initialization.

* **Write `0xFF` to Register `0x02` (INT\_STAT):** Writing a '1' to any bit in this register clears that interrupt flag.

After this sequence, the TCA8418 will be in its low-power idle mode, scanning the configured 4x10 matrix for key presses.

---

### **\#\# Interrupt Handling**

The interrupt mechanism alerts the host processor that a key event has occurred and needs to be read.

1. **Interrupt Trigger:** When a key is pressed or released, the TCA8418 pulls the **`INT` pin low**. Your processor should detect this falling edge and trigger an Interrupt Service Routine (ISR).  
2. **Interrupt Service Routine (ISR):** To maintain system responsiveness, the ISR should be minimal. Its only job should be to set a software flag (e.g., `key_event_pending = true`) and then exit. The main application loop will handle the I2C communication.  
3. **Main Loop Processing:** When the main loop sees that the `key_event_pending` flag is set, it should:  
   * **Read Register `0x02` (INT\_STAT):** Check the interrupt source. If  
      **Bit 0 (K\_INT)** is '1', it confirms a key press/release event occurred.  
   * **Proceed to Read the Keys** (as described in the next section).  
   * **Clear the Interrupt:** After the FIFO has been completely emptied, write a '1' to the `K_INT` bit in the `INT_STAT` register to clear the flag. This will cause the  
      `INT` pin to be released (go high).

---

### **\#\# Reading Keys from the FIFO**

This procedure reads the sequence of key events that triggered the interrupt. The TCA8418 has a

**10-byte FIFO** that can store up to 10 key press and release events.

1. **Check Event Count:**  
   * Read **Register `0x03` (KEY\_LCK\_EC)**. Bits \[3:0\] of the returned byte indicate how many events are currently stored in the FIFO.  
2. **Read FIFO Events:**  
   * Loop for the number of events reported in the previous step.  
   * In each loop iteration, perform an  
      **I2C read from Register `0x04` (KEY\_EVENT\_A)**.  
   * Each read from this specific address automatically pops the oldest event from the FIFO, and the event counter in  
      `KEY_LCK_EC` is decremented.  
3. **Decode Each Event Byte:** For each byte read from the FIFO:  
   * **Bit 7** indicates the event type: **'1' for a press**, **'0' for a release**.  
   * **Bits \[6:0\]** contain the key value (1-80 for a matrix key). This value corresponds to the key's position in the  
      **Key Event Table**. Your firmware will need a map to translate this value (e.g., decimal 12\) into a QWERTY character (e.g., 'E').  
4. **Finalize:**  
   * After the loop completes, the FIFO is empty.  
   * Clear the interrupt source bit in the `INT_STAT` register as described in the interrupt handling section.


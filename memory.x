MEMORY
{
    /* NOTE K = KiBi = 1024 bytes */
    /* STM32F401xB/C and STM32F401xD/E */
    FLASH (rx) : ORIGIN = 0x0800000, LENGTH = 256K
    RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 64K
}
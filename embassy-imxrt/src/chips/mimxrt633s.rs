pub use mimxrt633s_pac as pac;

#[allow(clippy::missing_safety_doc)]
pub mod interrupts {
    embassy_hal_internal::interrupt_mod!(
        ACMP,
        ADC0,
        CASPER,
        CTIMER0,
        CTIMER1,
        CTIMER2,
        CTIMER3,
        CTIMER4,
        DMA0,
        DMA1,
        DMIC0,
        ESPI,
        FLEXCOMM0,
        FLEXCOMM1,
        FLEXCOMM14,
        FLEXCOMM15,
        FLEXCOMM2,
        FLEXCOMM3,
        FLEXCOMM4,
        FLEXCOMM5,
        FLEXCOMM6,
        FLEXCOMM7,
        FLEXSPI,
        GPIO_INTA,
        GPIO_INTB,
        HASHCRYPT,
        HWVAD0,
        HYPERVISOR,
        I3C0,
        MRT0,
        MU_A,
        OS_EVENT,
        PIN_INT0,
        PIN_INT1,
        PIN_INT2,
        PIN_INT3,
        PIN_INT4,
        PIN_INT5,
        PIN_INT6,
        PIN_INT7,
        PMC_PMIC,
        POWERQUAD,
        PUF,
        RNG,
        RTC,
        SCT0,
        SECUREVIOLATION,
        SGPIO_INTA,
        SGPIO_INTB,
        USB,
        USBPHY_DCD,
        USB_WAKEUP,
        USDHC0,
        USDHC1,
        UTICK0,
        WDT0,
        WDT1,
    );
}

embassy_hal_internal::peripherals!(
    ACMP,
    ADC0,
    CASPER,
    CRC,
    CTIMER0_COUNT_CHANNEL0,
    CTIMER0_COUNT_CHANNEL1,
    CTIMER0_COUNT_CHANNEL2,
    CTIMER0_COUNT_CHANNEL3,
    CTIMER0_CAPTURE_CHANNEL0,
    CTIMER0_CAPTURE_CHANNEL1,
    CTIMER0_CAPTURE_CHANNEL2,
    CTIMER0_CAPTURE_CHANNEL3,
    CTIMER1_COUNT_CHANNEL0,
    CTIMER1_COUNT_CHANNEL1,
    CTIMER1_COUNT_CHANNEL2,
    CTIMER1_COUNT_CHANNEL3,
    CTIMER1_CAPTURE_CHANNEL0,
    CTIMER1_CAPTURE_CHANNEL1,
    CTIMER1_CAPTURE_CHANNEL2,
    CTIMER1_CAPTURE_CHANNEL3,
    CTIMER2_COUNT_CHANNEL0,
    CTIMER2_COUNT_CHANNEL1,
    CTIMER2_COUNT_CHANNEL2,
    CTIMER2_COUNT_CHANNEL3,
    CTIMER2_CAPTURE_CHANNEL0,
    CTIMER2_CAPTURE_CHANNEL1,
    CTIMER2_CAPTURE_CHANNEL2,
    CTIMER2_CAPTURE_CHANNEL3,
    CTIMER3_COUNT_CHANNEL0,
    CTIMER3_COUNT_CHANNEL1,
    CTIMER3_COUNT_CHANNEL2,
    CTIMER3_COUNT_CHANNEL3,
    CTIMER3_CAPTURE_CHANNEL0,
    CTIMER3_CAPTURE_CHANNEL1,
    CTIMER3_CAPTURE_CHANNEL2,
    CTIMER3_CAPTURE_CHANNEL3,
    CTIMER4_COUNT_CHANNEL0,
    CTIMER4_COUNT_CHANNEL1,
    CTIMER4_COUNT_CHANNEL2,
    CTIMER4_COUNT_CHANNEL3,
    CTIMER4_CAPTURE_CHANNEL0,
    CTIMER4_CAPTURE_CHANNEL1,
    CTIMER4_CAPTURE_CHANNEL2,
    CTIMER4_CAPTURE_CHANNEL3,
    DMA0,
    DMA0_CH0,
    DMA0_CH1,
    DMA0_CH2,
    DMA0_CH3,
    DMA0_CH4,
    DMA0_CH5,
    DMA0_CH6,
    DMA0_CH7,
    DMA0_CH8,
    DMA0_CH9,
    DMA0_CH10,
    DMA0_CH11,
    DMA0_CH12,
    DMA0_CH13,
    DMA0_CH14,
    DMA0_CH15,
    DMA0_CH16,
    DMA0_CH17,
    DMA0_CH18,
    DMA0_CH19,
    DMA0_CH20,
    DMA0_CH21,
    DMA0_CH22,
    DMA0_CH23,
    DMA0_CH24,
    DMA0_CH25,
    DMA0_CH26,
    DMA0_CH27,
    DMA0_CH28,
    DMA0_CH29,
    DMA0_CH30,
    DMA0_CH31,
    DMA0_CH32,
    DMA1,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA1_CH8,
    DMA1_CH9,
    DMA1_CH10,
    DMA1_CH11,
    DMA1_CH12,
    DMA1_CH13,
    DMA1_CH14,
    DMA1_CH15,
    DMA1_CH16,
    DMA1_CH17,
    DMA1_CH18,
    DMA1_CH19,
    DMA1_CH20,
    DMA1_CH21,
    DMA1_CH22,
    DMA1_CH23,
    DMA1_CH24,
    DMA1_CH25,
    DMA1_CH26,
    DMA1_CH27,
    DMA1_CH28,
    DMA1_CH29,
    DMA1_CH30,
    DMA1_CH31,
    DMA1_CH32,
    DMIC0,
    DSPWAKE,
    ESPI,
    FLEXCOMM0,
    FLEXCOMM1,
    FLEXCOMM14,
    FLEXCOMM15,
    FLEXCOMM2,
    FLEXCOMM3,
    FLEXCOMM4,
    FLEXCOMM5,
    FLEXCOMM6,
    FLEXCOMM7,
    FLEXSPI,
    FREQME,
    GPIO_INTA,
    GPIO_INTB,
    HASHCRYPT,
    HSGPIO0,
    HSGPIO1,
    HSGPIO2,
    HSGPIO3,
    HSGPIO4,
    HSGPIO5,
    HSGPIO6,
    HSGPIO7,
    HWVAD0,
    HYPERVISOR,
    I3C0,
    MRT0,
    MU_A,
    OS_EVENT,
    PIN_INT0,
    PIN_INT1,
    PIN_INT2,
    PIN_INT3,
    PIN_INT4,
    PIN_INT5,
    PIN_INT6,
    PIN_INT7,
    PIO0_0,
    PIO0_1,
    PIO0_10,
    PIO0_11,
    PIO0_12,
    PIO0_13,
    PIO0_14,
    PIO0_15,
    PIO0_16,
    PIO0_17,
    PIO0_18,
    PIO0_19,
    PIO0_2,
    PIO0_20,
    PIO0_21,
    PIO0_22,
    PIO0_23,
    PIO0_24,
    PIO0_25,
    PIO0_26,
    PIO0_27,
    PIO0_28,
    PIO0_29,
    PIO0_3,
    PIO0_30,
    PIO0_31,
    PIO0_4,
    PIO0_5,
    PIO0_6,
    PIO0_7,
    PIO0_8,
    PIO0_9,
    PIO1_0,
    PIO1_1,
    PIO1_10,
    PIO1_11,
    PIO1_12,
    PIO1_13,
    PIO1_14,
    PIO1_15,
    PIO1_16,
    PIO1_17,
    PIO1_18,
    PIO1_19,
    PIO1_2,
    PIO1_20,
    PIO1_21,
    PIO1_22,
    PIO1_23,
    PIO1_24,
    PIO1_25,
    PIO1_26,
    PIO1_27,
    PIO1_28,
    PIO1_29,
    PIO1_3,
    PIO1_30,
    PIO1_31,
    PIO1_4,
    PIO1_5,
    PIO1_6,
    PIO1_7,
    PIO1_8,
    PIO1_9,
    PIO2_0,
    PIO2_1,
    PIO2_10,
    PIO2_11,
    PIO2_12,
    PIO2_13,
    PIO2_14,
    PIO2_15,
    PIO2_16,
    PIO2_17,
    PIO2_18,
    PIO2_19,
    PIO2_2,
    PIO2_20,
    PIO2_21,
    PIO2_22,
    PIO2_23,
    PIO2_24,
    PIO2_25,
    PIO2_26,
    PIO2_27,
    PIO2_28,
    PIO2_29,
    PIO2_3,
    PIO2_30,
    PIO2_31,
    PIO2_4,
    PIO2_5,
    PIO2_6,
    PIO2_7,
    PIO2_8,
    PIO2_9,
    PIO3_0,
    PIO3_1,
    PIO3_10,
    PIO3_11,
    PIO3_12,
    PIO3_13,
    PIO3_14,
    PIO3_15,
    PIO3_16,
    PIO3_17,
    PIO3_18,
    PIO3_19,
    PIO3_2,
    PIO3_20,
    PIO3_21,
    PIO3_22,
    PIO3_23,
    PIO3_24,
    PIO3_25,
    PIO3_26,
    PIO3_27,
    PIO3_28,
    PIO3_29,
    PIO3_3,
    PIO3_30,
    PIO3_31,
    PIO3_4,
    PIO3_5,
    PIO3_6,
    PIO3_7,
    PIO3_8,
    PIO3_9,
    PIO4_0,
    PIO4_1,
    PIO4_10,
    PIO4_2,
    PIO4_3,
    PIO4_4,
    PIO4_5,
    PIO4_6,
    PIO4_7,
    PIO4_8,
    PIO4_9,
    PIO7_24,
    PIO7_25,
    PIO7_26,
    PIO7_27,
    PIO7_28,
    PIO7_29,
    PIO7_30,
    PIO7_31,
    PIOFC15_SCL,
    PIOFC15_SDA,
    PMC_PMIC,
    PIMCTL,
    POWERQUAD,
    PUF,
    RNG,
    RTC,
    SCT0,
    SECGPIO,
    SECUREVIOLATION,
    SEMA42,
    SGPIO_INTA,
    SGPIO_INTB,
    USBHSD,
    USBHSH,
    USBPHY,
    USB_WAKEUP,
    USDHC0,
    USDHC1,
    UTICK0,
    WDT0,
    WDT1,
);

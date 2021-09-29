#[doc = "Register `CLOCKENSTAT` reader"]
pub struct R(crate::R<CLOCKENSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCKENSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCKENSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCKENSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCKENSTAT` writer"]
pub struct W(crate::W<CLOCKENSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKENSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLOCKENSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCKENSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock enable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKENSTAT_A {
    #[doc = "1: Clock enable for the ADC. value."]
    ADC_CLKEN = 1,
    #[doc = "2: Clock enable for the APBDMA ACTIVITY value."]
    APBDMA_ACTIVITY_CLKEN = 2,
    #[doc = "4: Clock enable for the APBDMA AOH DOMAIN value."]
    APBDMA_AOH_CLKEN = 4,
    #[doc = "8: Clock enable for the APBDMA AOL DOMAIN value."]
    APBDMA_AOL_CLKEN = 8,
    #[doc = "16: Clock enable for the APBDMA_APB value."]
    APBDMA_APB_CLKEN = 16,
    #[doc = "32: Clock enable for the APBDMA_BLEL value."]
    APBDMA_BLEL_CLKEN = 32,
    #[doc = "64: Clock enable for the APBDMA_HCPA value."]
    APBDMA_HCPA_CLKEN = 64,
    #[doc = "128: Clock enable for the APBDMA_HCPB value."]
    APBDMA_HCPB_CLKEN = 128,
    #[doc = "256: Clock enable for the APBDMA_HCPC value."]
    APBDMA_HCPC_CLKEN = 256,
    #[doc = "512: Clock enable for the APBDMA_MSPI value."]
    APBDMA_MSPI_CLKEN = 512,
    #[doc = "1024: Clock enable for the APBDMA_PDM value."]
    APBDMA_PDM_CLKEN = 1024,
    #[doc = "2048: Clock enable for the BLEIF value."]
    BLEIF_CLK_CLKEN = 2048,
    #[doc = "4096: Clock enable for the BLEIF 32khZ CLOCK value."]
    BLEIF_CLK32K_CLKEN = 4096,
    #[doc = "8192: Clock enable for the CTIMER BLOCK value."]
    CTIMER_CLKEN = 8192,
    #[doc = "16384: Clock enable for the CTIMER0A value."]
    CTIMER0A_CLKEN = 16384,
    #[doc = "32768: Clock enable for the CTIMER0B value."]
    CTIMER0B_CLKEN = 32768,
    #[doc = "65536: Clock enable for the CTIMER1A value."]
    CTIMER1A_CLKEN = 65536,
    #[doc = "131072: Clock enable for the CTIMER1B value."]
    CTIMER1B_CLKEN = 131072,
    #[doc = "262144: Clock enable for the CTIMER2A value."]
    CTIMER2A_CLKEN = 262144,
    #[doc = "524288: Clock enable for the CTIMER2B value."]
    CTIMER2B_CLKEN = 524288,
    #[doc = "1048576: Clock enable for the CTIMER3A value."]
    CTIMER3A_CLKEN = 1048576,
    #[doc = "2097152: Clock enable for the CTIMER3B value."]
    CTIMER3B_CLKEN = 2097152,
    #[doc = "4194304: Clock enable for the CTIMER4A value."]
    CTIMER4A_CLKEN = 4194304,
    #[doc = "8388608: Clock enable for the CTIMER4B value."]
    CTIMER4B_CLKEN = 8388608,
    #[doc = "16777216: Clock enable for the CTIMER5A value."]
    CTIMER5A_CLKEN = 16777216,
    #[doc = "33554432: Clock enable for the CTIMER5B value."]
    CTIMER5B_CLKEN = 33554432,
    #[doc = "67108864: Clock enable for the CTIMER6A value."]
    CTIMER6A_CLKEN = 67108864,
    #[doc = "134217728: Clock enable for the CTIMER6B value."]
    CTIMER6B_CLKEN = 134217728,
    #[doc = "268435456: Clock enable for the CTIMER7A value."]
    CTIMER7A_CLKEN = 268435456,
    #[doc = "536870912: Clock enable for the CTIMER7B value."]
    CTIMER7B_CLKEN = 536870912,
    #[doc = "1073741824: Clock enable for the DAP value."]
    DAP_CLKEN = 1073741824,
    #[doc = "2147483648: Clock enable for the IOMSTRIFC0 value."]
    IOMSTRIFC0_CLKEN = 2147483648,
}
impl From<CLOCKENSTAT_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKENSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLOCKENSTAT` reader - Clock enable status"]
pub struct CLOCKENSTAT_R(crate::FieldReader<u32, CLOCKENSTAT_A>);
impl CLOCKENSTAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLOCKENSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCKENSTAT_A> {
        match self.bits {
            1 => Some(CLOCKENSTAT_A::ADC_CLKEN),
            2 => Some(CLOCKENSTAT_A::APBDMA_ACTIVITY_CLKEN),
            4 => Some(CLOCKENSTAT_A::APBDMA_AOH_CLKEN),
            8 => Some(CLOCKENSTAT_A::APBDMA_AOL_CLKEN),
            16 => Some(CLOCKENSTAT_A::APBDMA_APB_CLKEN),
            32 => Some(CLOCKENSTAT_A::APBDMA_BLEL_CLKEN),
            64 => Some(CLOCKENSTAT_A::APBDMA_HCPA_CLKEN),
            128 => Some(CLOCKENSTAT_A::APBDMA_HCPB_CLKEN),
            256 => Some(CLOCKENSTAT_A::APBDMA_HCPC_CLKEN),
            512 => Some(CLOCKENSTAT_A::APBDMA_MSPI_CLKEN),
            1024 => Some(CLOCKENSTAT_A::APBDMA_PDM_CLKEN),
            2048 => Some(CLOCKENSTAT_A::BLEIF_CLK_CLKEN),
            4096 => Some(CLOCKENSTAT_A::BLEIF_CLK32K_CLKEN),
            8192 => Some(CLOCKENSTAT_A::CTIMER_CLKEN),
            16384 => Some(CLOCKENSTAT_A::CTIMER0A_CLKEN),
            32768 => Some(CLOCKENSTAT_A::CTIMER0B_CLKEN),
            65536 => Some(CLOCKENSTAT_A::CTIMER1A_CLKEN),
            131072 => Some(CLOCKENSTAT_A::CTIMER1B_CLKEN),
            262144 => Some(CLOCKENSTAT_A::CTIMER2A_CLKEN),
            524288 => Some(CLOCKENSTAT_A::CTIMER2B_CLKEN),
            1048576 => Some(CLOCKENSTAT_A::CTIMER3A_CLKEN),
            2097152 => Some(CLOCKENSTAT_A::CTIMER3B_CLKEN),
            4194304 => Some(CLOCKENSTAT_A::CTIMER4A_CLKEN),
            8388608 => Some(CLOCKENSTAT_A::CTIMER4B_CLKEN),
            16777216 => Some(CLOCKENSTAT_A::CTIMER5A_CLKEN),
            33554432 => Some(CLOCKENSTAT_A::CTIMER5B_CLKEN),
            67108864 => Some(CLOCKENSTAT_A::CTIMER6A_CLKEN),
            134217728 => Some(CLOCKENSTAT_A::CTIMER6B_CLKEN),
            268435456 => Some(CLOCKENSTAT_A::CTIMER7A_CLKEN),
            536870912 => Some(CLOCKENSTAT_A::CTIMER7B_CLKEN),
            1073741824 => Some(CLOCKENSTAT_A::DAP_CLKEN),
            2147483648 => Some(CLOCKENSTAT_A::IOMSTRIFC0_CLKEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_CLKEN`"]
    #[inline(always)]
    pub fn is_adc_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::ADC_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_ACTIVITY_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_activity_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_ACTIVITY_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_AOH_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_aoh_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_AOH_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_AOL_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_aol_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_AOL_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_APB_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_apb_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_APB_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_BLEL_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_blel_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_BLEL_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_HCPA_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_hcpa_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_HCPA_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_HCPB_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_hcpb_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_HCPB_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_HCPC_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_hcpc_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_HCPC_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_MSPI_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_mspi_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_MSPI_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_PDM_CLKEN`"]
    #[inline(always)]
    pub fn is_apbdma_pdm_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::APBDMA_PDM_CLKEN
    }
    #[doc = "Checks if the value of the field is `BLEIF_CLK_CLKEN`"]
    #[inline(always)]
    pub fn is_bleif_clk_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::BLEIF_CLK_CLKEN
    }
    #[doc = "Checks if the value of the field is `BLEIF_CLK32K_CLKEN`"]
    #[inline(always)]
    pub fn is_bleif_clk32k_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::BLEIF_CLK32K_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER0A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer0a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER0A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER0B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer0b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER0B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER1A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer1a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER1A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER1B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer1b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER1B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER2A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer2a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER2A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER2B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer2b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER2B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER3A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer3a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER3A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER3B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer3b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER3B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER4A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer4a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER4A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER4B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer4b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER4B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER5A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer5a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER5A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER5B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer5b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER5B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER6A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer6a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER6A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER6B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer6b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER6B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER7A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer7a_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER7A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER7B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer7b_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::CTIMER7B_CLKEN
    }
    #[doc = "Checks if the value of the field is `DAP_CLKEN`"]
    #[inline(always)]
    pub fn is_dap_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::DAP_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC0_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc0_clken(&self) -> bool {
        **self == CLOCKENSTAT_A::IOMSTRIFC0_CLKEN
    }
}
impl core::ops::Deref for CLOCKENSTAT_R {
    type Target = crate::FieldReader<u32, CLOCKENSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCKENSTAT` writer - Clock enable status"]
pub struct CLOCKENSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKENSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKENSTAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock enable for the ADC. value."]
    #[inline(always)]
    pub fn adc_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::ADC_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA ACTIVITY value."]
    #[inline(always)]
    pub fn apbdma_activity_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_ACTIVITY_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA AOH DOMAIN value."]
    #[inline(always)]
    pub fn apbdma_aoh_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_AOH_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA AOL DOMAIN value."]
    #[inline(always)]
    pub fn apbdma_aol_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_AOL_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_APB value."]
    #[inline(always)]
    pub fn apbdma_apb_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_APB_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_BLEL value."]
    #[inline(always)]
    pub fn apbdma_blel_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_BLEL_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_HCPA value."]
    #[inline(always)]
    pub fn apbdma_hcpa_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_HCPA_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_HCPB value."]
    #[inline(always)]
    pub fn apbdma_hcpb_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_HCPB_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_HCPC value."]
    #[inline(always)]
    pub fn apbdma_hcpc_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_HCPC_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_MSPI value."]
    #[inline(always)]
    pub fn apbdma_mspi_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_MSPI_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_PDM value."]
    #[inline(always)]
    pub fn apbdma_pdm_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::APBDMA_PDM_CLKEN)
    }
    #[doc = "Clock enable for the BLEIF value."]
    #[inline(always)]
    pub fn bleif_clk_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::BLEIF_CLK_CLKEN)
    }
    #[doc = "Clock enable for the BLEIF 32khZ CLOCK value."]
    #[inline(always)]
    pub fn bleif_clk32k_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::BLEIF_CLK32K_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER BLOCK value."]
    #[inline(always)]
    pub fn ctimer_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER0A value."]
    #[inline(always)]
    pub fn ctimer0a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER0A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER0B value."]
    #[inline(always)]
    pub fn ctimer0b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER0B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER1A value."]
    #[inline(always)]
    pub fn ctimer1a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER1A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER1B value."]
    #[inline(always)]
    pub fn ctimer1b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER1B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER2A value."]
    #[inline(always)]
    pub fn ctimer2a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER2A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER2B value."]
    #[inline(always)]
    pub fn ctimer2b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER2B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER3A value."]
    #[inline(always)]
    pub fn ctimer3a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER3A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER3B value."]
    #[inline(always)]
    pub fn ctimer3b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER3B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER4A value."]
    #[inline(always)]
    pub fn ctimer4a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER4A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER4B value."]
    #[inline(always)]
    pub fn ctimer4b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER4B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER5A value."]
    #[inline(always)]
    pub fn ctimer5a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER5A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER5B value."]
    #[inline(always)]
    pub fn ctimer5b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER5B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER6A value."]
    #[inline(always)]
    pub fn ctimer6a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER6A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER6B value."]
    #[inline(always)]
    pub fn ctimer6b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER6B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER7A value."]
    #[inline(always)]
    pub fn ctimer7a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER7A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER7B value."]
    #[inline(always)]
    pub fn ctimer7b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::CTIMER7B_CLKEN)
    }
    #[doc = "Clock enable for the DAP value."]
    #[inline(always)]
    pub fn dap_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::DAP_CLKEN)
    }
    #[doc = "Clock enable for the IOMSTRIFC0 value."]
    #[inline(always)]
    pub fn iomstrifc0_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTAT_A::IOMSTRIFC0_CLKEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline(always)]
    pub fn clockenstat(&self) -> CLOCKENSTAT_R {
        CLOCKENSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline(always)]
    pub fn clockenstat(&mut self) -> CLOCKENSTAT_W {
        CLOCKENSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Enable Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockenstat](index.html) module"]
pub struct CLOCKENSTAT_SPEC;
impl crate::RegisterSpec for CLOCKENSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clockenstat::R](R) reader structure"]
impl crate::Readable for CLOCKENSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clockenstat::W](W) writer structure"]
impl crate::Writable for CLOCKENSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCKENSTAT to value 0"]
impl crate::Resettable for CLOCKENSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

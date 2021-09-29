#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Error Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DERR_A {
    #[doc = "1: DMA Error Condition Occurred value."]
    DMAERROR = 1,
}
impl From<DERR_A> for bool {
    #[inline(always)]
    fn from(variant: DERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DERR` reader - DMA Error Condition"]
pub struct DERR_R(crate::FieldReader<bool, DERR_A>);
impl DERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DERR_A> {
        match self.bits {
            true => Some(DERR_A::DMAERROR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMAERROR`"]
    #[inline(always)]
    pub fn is_dmaerror(&self) -> bool {
        **self == DERR_A::DMAERROR
    }
}
impl core::ops::Deref for DERR_R {
    type Target = crate::FieldReader<bool, DERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERR` writer - DMA Error Condition"]
pub struct DERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA Error Condition Occurred value."]
    #[inline(always)]
    pub fn dmaerror(self) -> &'a mut W {
        self.variant(DERR_A::DMAERROR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "DMA Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMP_A {
    #[doc = "1: DMA Completed a transfer value."]
    DMACOMPLETE = 1,
}
impl From<DCMP_A> for bool {
    #[inline(always)]
    fn from(variant: DCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMP` reader - DMA Transfer Complete"]
pub struct DCMP_R(crate::FieldReader<bool, DCMP_A>);
impl DCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCMP_A> {
        match self.bits {
            true => Some(DCMP_A::DMACOMPLETE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMACOMPLETE`"]
    #[inline(always)]
    pub fn is_dmacomplete(&self) -> bool {
        **self == DCMP_A::DMACOMPLETE
    }
}
impl core::ops::Deref for DCMP_R {
    type Target = crate::FieldReader<bool, DCMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMP` writer - DMA Transfer Complete"]
pub struct DCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA Completed a transfer value."]
    #[inline(always)]
    pub fn dmacomplete(self) -> &'a mut W {
        self.variant(DCMP_A::DMACOMPLETE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Window comparator voltage incursion interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCINC_A {
    #[doc = "1: Window comparitor voltage incursion interrupt. value."]
    WCINCINT = 1,
}
impl From<WCINC_A> for bool {
    #[inline(always)]
    fn from(variant: WCINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCINC` reader - Window comparator voltage incursion interrupt."]
pub struct WCINC_R(crate::FieldReader<bool, WCINC_A>);
impl WCINC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WCINC_A> {
        match self.bits {
            true => Some(WCINC_A::WCINCINT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WCINCINT`"]
    #[inline(always)]
    pub fn is_wcincint(&self) -> bool {
        **self == WCINC_A::WCINCINT
    }
}
impl core::ops::Deref for WCINC_R {
    type Target = crate::FieldReader<bool, WCINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCINC` writer - Window comparator voltage incursion interrupt."]
pub struct WCINC_W<'a> {
    w: &'a mut W,
}
impl<'a> WCINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCINC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Window comparitor voltage incursion interrupt. value."]
    #[inline(always)]
    pub fn wcincint(self) -> &'a mut W {
        self.variant(WCINC_A::WCINCINT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Window comparator voltage excursion interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEXC_A {
    #[doc = "1: Window comparitor voltage excursion interrupt. value."]
    WCEXCINT = 1,
}
impl From<WCEXC_A> for bool {
    #[inline(always)]
    fn from(variant: WCEXC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEXC` reader - Window comparator voltage excursion interrupt."]
pub struct WCEXC_R(crate::FieldReader<bool, WCEXC_A>);
impl WCEXC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCEXC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WCEXC_A> {
        match self.bits {
            true => Some(WCEXC_A::WCEXCINT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WCEXCINT`"]
    #[inline(always)]
    pub fn is_wcexcint(&self) -> bool {
        **self == WCEXC_A::WCEXCINT
    }
}
impl core::ops::Deref for WCEXC_R {
    type Target = crate::FieldReader<bool, WCEXC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCEXC` writer - Window comparator voltage excursion interrupt."]
pub struct WCEXC_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEXC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEXC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Window comparitor voltage excursion interrupt. value."]
    #[inline(always)]
    pub fn wcexcint(self) -> &'a mut W {
        self.variant(WCEXC_A::WCEXCINT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "FIFO 100 percent full interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOOVR2_A {
    #[doc = "1: FIFO 100 percent full interrupt. value."]
    FIFOFULLINT = 1,
}
impl From<FIFOOVR2_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVR2` reader - FIFO 100 percent full interrupt."]
pub struct FIFOOVR2_R(crate::FieldReader<bool, FIFOOVR2_A>);
impl FIFOOVR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOOVR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIFOOVR2_A> {
        match self.bits {
            true => Some(FIFOOVR2_A::FIFOFULLINT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FIFOFULLINT`"]
    #[inline(always)]
    pub fn is_fifofullint(&self) -> bool {
        **self == FIFOOVR2_A::FIFOFULLINT
    }
}
impl core::ops::Deref for FIFOOVR2_R {
    type Target = crate::FieldReader<bool, FIFOOVR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOOVR2` writer - FIFO 100 percent full interrupt."]
pub struct FIFOOVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOOVR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOOVR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO 100 percent full interrupt. value."]
    #[inline(always)]
    pub fn fifofullint(self) -> &'a mut W {
        self.variant(FIFOOVR2_A::FIFOFULLINT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "FIFO 75 percent full interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOOVR1_A {
    #[doc = "1: FIFO 75 percent full interrupt. value."]
    FIFO75INT = 1,
}
impl From<FIFOOVR1_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVR1` reader - FIFO 75 percent full interrupt."]
pub struct FIFOOVR1_R(crate::FieldReader<bool, FIFOOVR1_A>);
impl FIFOOVR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOOVR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIFOOVR1_A> {
        match self.bits {
            true => Some(FIFOOVR1_A::FIFO75INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO75INT`"]
    #[inline(always)]
    pub fn is_fifo75int(&self) -> bool {
        **self == FIFOOVR1_A::FIFO75INT
    }
}
impl core::ops::Deref for FIFOOVR1_R {
    type Target = crate::FieldReader<bool, FIFOOVR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOOVR1` writer - FIFO 75 percent full interrupt."]
pub struct FIFOOVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOOVR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOOVR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO 75 percent full interrupt. value."]
    #[inline(always)]
    pub fn fifo75int(self) -> &'a mut W {
        self.variant(FIFOOVR1_A::FIFO75INT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "ADC scan complete interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCNCMP_A {
    #[doc = "1: ADC scan complete interrupt. value."]
    SCNCMPINT = 1,
}
impl From<SCNCMP_A> for bool {
    #[inline(always)]
    fn from(variant: SCNCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCNCMP` reader - ADC scan complete interrupt."]
pub struct SCNCMP_R(crate::FieldReader<bool, SCNCMP_A>);
impl SCNCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCNCMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCNCMP_A> {
        match self.bits {
            true => Some(SCNCMP_A::SCNCMPINT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCNCMPINT`"]
    #[inline(always)]
    pub fn is_scncmpint(&self) -> bool {
        **self == SCNCMP_A::SCNCMPINT
    }
}
impl core::ops::Deref for SCNCMP_R {
    type Target = crate::FieldReader<bool, SCNCMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCNCMP` writer - ADC scan complete interrupt."]
pub struct SCNCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCNCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCNCMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC scan complete interrupt. value."]
    #[inline(always)]
    pub fn scncmpint(self) -> &'a mut W {
        self.variant(SCNCMP_A::SCNCMPINT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "ADC conversion complete interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNVCMP_A {
    #[doc = "1: ADC conversion complete interrupt. value."]
    CNVCMPINT = 1,
}
impl From<CNVCMP_A> for bool {
    #[inline(always)]
    fn from(variant: CNVCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNVCMP` reader - ADC conversion complete interrupt."]
pub struct CNVCMP_R(crate::FieldReader<bool, CNVCMP_A>);
impl CNVCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNVCMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNVCMP_A> {
        match self.bits {
            true => Some(CNVCMP_A::CNVCMPINT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CNVCMPINT`"]
    #[inline(always)]
    pub fn is_cnvcmpint(&self) -> bool {
        **self == CNVCMP_A::CNVCMPINT
    }
}
impl core::ops::Deref for CNVCMP_R {
    type Target = crate::FieldReader<bool, CNVCMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNVCMP` writer - ADC conversion complete interrupt."]
pub struct CNVCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CNVCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNVCMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC conversion complete interrupt. value."]
    #[inline(always)]
    pub fn cnvcmpint(self) -> &'a mut W {
        self.variant(CNVCMP_A::CNVCMPINT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - DMA Error Condition"]
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA Transfer Complete"]
    #[inline(always)]
    pub fn dcmp(&self) -> DCMP_R {
        DCMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcinc(&self) -> WCINC_R {
        WCINC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexc(&self) -> WCEXC_R {
        WCEXC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr2(&self) -> FIFOOVR2_R {
        FIFOOVR2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr1(&self) -> FIFOOVR1_R {
        FIFOOVR1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmp(&self) -> SCNCMP_R {
        SCNCMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmp(&self) -> CNVCMP_R {
        CNVCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - DMA Error Condition"]
    #[inline(always)]
    pub fn derr(&mut self) -> DERR_W {
        DERR_W { w: self }
    }
    #[doc = "Bit 6 - DMA Transfer Complete"]
    #[inline(always)]
    pub fn dcmp(&mut self) -> DCMP_W {
        DCMP_W { w: self }
    }
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcinc(&mut self) -> WCINC_W {
        WCINC_W { w: self }
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexc(&mut self) -> WCEXC_W {
        WCEXC_W { w: self }
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr2(&mut self) -> FIFOOVR2_W {
        FIFOOVR2_W { w: self }
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr1(&mut self) -> FIFOOVR1_W {
        FIFOOVR1_W { w: self }
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmp(&mut self) -> SCNCMP_W {
        SCNCMP_W { w: self }
    }
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmp(&mut self) -> CNVCMP_W {
        CNVCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt registers: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

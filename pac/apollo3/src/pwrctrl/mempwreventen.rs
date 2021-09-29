#[doc = "Register `MEMPWREVENTEN` reader"]
pub struct R(crate::R<MEMPWREVENTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMPWREVENTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMPWREVENTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMPWREVENTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMPWREVENTEN` writer"]
pub struct W(crate::W<MEMPWREVENTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMPWREVENTEN_SPEC>;
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
impl From<crate::W<MEMPWREVENTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMPWREVENTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control CACHEB2 power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB2EN_A {
    #[doc = "1: Enable CACHE BANK 2 status event value."]
    EN = 1,
    #[doc = "0: Disable CACHE BANK 2 status event value."]
    DIS = 0,
}
impl From<CACHEB2EN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEB2EN` reader - Control CACHEB2 power-on status event"]
pub struct CACHEB2EN_R(crate::FieldReader<bool, CACHEB2EN_A>);
impl CACHEB2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEB2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB2EN_A {
        match self.bits {
            true => CACHEB2EN_A::EN,
            false => CACHEB2EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CACHEB2EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CACHEB2EN_A::DIS
    }
}
impl core::ops::Deref for CACHEB2EN_R {
    type Target = crate::FieldReader<bool, CACHEB2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEB2EN` writer - Control CACHEB2 power-on status event"]
pub struct CACHEB2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable CACHE BANK 2 status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB2EN_A::EN)
    }
    #[doc = "Disable CACHE BANK 2 status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB2EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Control CACHE BANK 0 power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB0EN_A {
    #[doc = "1: Enable CACHE BANK 0 status event value."]
    EN = 1,
    #[doc = "0: Disable CACHE BANK 0 status event value."]
    DIS = 0,
}
impl From<CACHEB0EN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEB0EN` reader - Control CACHE BANK 0 power-on status event"]
pub struct CACHEB0EN_R(crate::FieldReader<bool, CACHEB0EN_A>);
impl CACHEB0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEB0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB0EN_A {
        match self.bits {
            true => CACHEB0EN_A::EN,
            false => CACHEB0EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CACHEB0EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CACHEB0EN_A::DIS
    }
}
impl core::ops::Deref for CACHEB0EN_R {
    type Target = crate::FieldReader<bool, CACHEB0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEB0EN` writer - Control CACHE BANK 0 power-on status event"]
pub struct CACHEB0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable CACHE BANK 0 status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB0EN_A::EN)
    }
    #[doc = "Disable CACHE BANK 0 status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB0EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Control Flash power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1EN_A {
    #[doc = "1: Enable FLASH status event value."]
    EN = 1,
    #[doc = "0: Disables FLASH status event value."]
    DIS = 0,
}
impl From<FLASH1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH1EN` reader - Control Flash power-on status event"]
pub struct FLASH1EN_R(crate::FieldReader<bool, FLASH1EN_A>);
impl FLASH1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH1EN_A {
        match self.bits {
            true => FLASH1EN_A::EN,
            false => FLASH1EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FLASH1EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FLASH1EN_A::DIS
    }
}
impl core::ops::Deref for FLASH1EN_R {
    type Target = crate::FieldReader<bool, FLASH1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH1EN` writer - Control Flash power-on status event"]
pub struct FLASH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable FLASH status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1EN_A::EN)
    }
    #[doc = "Disables FLASH status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Control Flash power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0EN_A {
    #[doc = "1: Enable FLASH status event value."]
    EN = 1,
    #[doc = "0: Disables FLASH status event value."]
    DIS = 0,
}
impl From<FLASH0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH0EN` reader - Control Flash power-on status event"]
pub struct FLASH0EN_R(crate::FieldReader<bool, FLASH0EN_A>);
impl FLASH0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH0EN_A {
        match self.bits {
            true => FLASH0EN_A::EN,
            false => FLASH0EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FLASH0EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FLASH0EN_A::DIS
    }
}
impl core::ops::Deref for FLASH0EN_R {
    type Target = crate::FieldReader<bool, FLASH0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH0EN` writer - Control Flash power-on status event"]
pub struct FLASH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable FLASH status event value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0EN_A::EN)
    }
    #[doc = "Disables FLASH status event value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0EN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Control SRAM power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAMEN_A {
    #[doc = "0: Disable SRAM power-on status event value."]
    NONE = 0,
    #[doc = "1: Enable SRAM group0 (0KB-32KB) power on status event value."]
    GROUP0EN = 1,
    #[doc = "2: Enable SRAM group1 (32KB-64KB) power on status event value."]
    GROUP1EN = 2,
    #[doc = "4: Enable SRAM group2 (64KB-96KB) power on status event value."]
    GROUP2EN = 4,
    #[doc = "8: Enable SRAM group3 (96KB-128KB) power on status event value."]
    GROUP3EN = 8,
    #[doc = "16: Enable SRAM group4 (128KB-160KB) power on status event value."]
    GROUP4EN = 16,
    #[doc = "32: Enable SRAM group5 (160KB-192KB) power on status event value."]
    GROUP5EN = 32,
    #[doc = "64: Enable SRAM group6 (192KB-224KB) power on status event value."]
    GROUP6EN = 64,
    #[doc = "128: Enable SRAM group7 (224KB-256KB) power on status event value."]
    GROUP7EN = 128,
    #[doc = "256: Enable SRAM group8 (256KB-288KB) power on status event value."]
    GROUP8EN = 256,
    #[doc = "512: Enable SRAM group9 (288KB-320KB) power on status event value."]
    GROUP9EN = 512,
}
impl From<SRAMEN_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAMEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAMEN` reader - Control SRAM power-on status event"]
pub struct SRAMEN_R(crate::FieldReader<u16, SRAMEN_A>);
impl SRAMEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        SRAMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAMEN_A> {
        match self.bits {
            0 => Some(SRAMEN_A::NONE),
            1 => Some(SRAMEN_A::GROUP0EN),
            2 => Some(SRAMEN_A::GROUP1EN),
            4 => Some(SRAMEN_A::GROUP2EN),
            8 => Some(SRAMEN_A::GROUP3EN),
            16 => Some(SRAMEN_A::GROUP4EN),
            32 => Some(SRAMEN_A::GROUP5EN),
            64 => Some(SRAMEN_A::GROUP6EN),
            128 => Some(SRAMEN_A::GROUP7EN),
            256 => Some(SRAMEN_A::GROUP8EN),
            512 => Some(SRAMEN_A::GROUP9EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SRAMEN_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0EN`"]
    #[inline(always)]
    pub fn is_group0en(&self) -> bool {
        **self == SRAMEN_A::GROUP0EN
    }
    #[doc = "Checks if the value of the field is `GROUP1EN`"]
    #[inline(always)]
    pub fn is_group1en(&self) -> bool {
        **self == SRAMEN_A::GROUP1EN
    }
    #[doc = "Checks if the value of the field is `GROUP2EN`"]
    #[inline(always)]
    pub fn is_group2en(&self) -> bool {
        **self == SRAMEN_A::GROUP2EN
    }
    #[doc = "Checks if the value of the field is `GROUP3EN`"]
    #[inline(always)]
    pub fn is_group3en(&self) -> bool {
        **self == SRAMEN_A::GROUP3EN
    }
    #[doc = "Checks if the value of the field is `GROUP4EN`"]
    #[inline(always)]
    pub fn is_group4en(&self) -> bool {
        **self == SRAMEN_A::GROUP4EN
    }
    #[doc = "Checks if the value of the field is `GROUP5EN`"]
    #[inline(always)]
    pub fn is_group5en(&self) -> bool {
        **self == SRAMEN_A::GROUP5EN
    }
    #[doc = "Checks if the value of the field is `GROUP6EN`"]
    #[inline(always)]
    pub fn is_group6en(&self) -> bool {
        **self == SRAMEN_A::GROUP6EN
    }
    #[doc = "Checks if the value of the field is `GROUP7EN`"]
    #[inline(always)]
    pub fn is_group7en(&self) -> bool {
        **self == SRAMEN_A::GROUP7EN
    }
    #[doc = "Checks if the value of the field is `GROUP8EN`"]
    #[inline(always)]
    pub fn is_group8en(&self) -> bool {
        **self == SRAMEN_A::GROUP8EN
    }
    #[doc = "Checks if the value of the field is `GROUP9EN`"]
    #[inline(always)]
    pub fn is_group9en(&self) -> bool {
        **self == SRAMEN_A::GROUP9EN
    }
}
impl core::ops::Deref for SRAMEN_R {
    type Target = crate::FieldReader<u16, SRAMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMEN` writer - Control SRAM power-on status event"]
pub struct SRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable SRAM power-on status event value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMEN_A::NONE)
    }
    #[doc = "Enable SRAM group0 (0KB-32KB) power on status event value."]
    #[inline(always)]
    pub fn group0en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP0EN)
    }
    #[doc = "Enable SRAM group1 (32KB-64KB) power on status event value."]
    #[inline(always)]
    pub fn group1en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP1EN)
    }
    #[doc = "Enable SRAM group2 (64KB-96KB) power on status event value."]
    #[inline(always)]
    pub fn group2en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP2EN)
    }
    #[doc = "Enable SRAM group3 (96KB-128KB) power on status event value."]
    #[inline(always)]
    pub fn group3en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP3EN)
    }
    #[doc = "Enable SRAM group4 (128KB-160KB) power on status event value."]
    #[inline(always)]
    pub fn group4en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP4EN)
    }
    #[doc = "Enable SRAM group5 (160KB-192KB) power on status event value."]
    #[inline(always)]
    pub fn group5en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP5EN)
    }
    #[doc = "Enable SRAM group6 (192KB-224KB) power on status event value."]
    #[inline(always)]
    pub fn group6en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP6EN)
    }
    #[doc = "Enable SRAM group7 (224KB-256KB) power on status event value."]
    #[inline(always)]
    pub fn group7en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP7EN)
    }
    #[doc = "Enable SRAM group8 (256KB-288KB) power on status event value."]
    #[inline(always)]
    pub fn group8en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP8EN)
    }
    #[doc = "Enable SRAM group9 (288KB-320KB) power on status event value."]
    #[inline(always)]
    pub fn group9en(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP9EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 3)) | ((value as u32 & 0x03ff) << 3);
        self.w
    }
}
#[doc = "Enable DTCM power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTCMEN_A {
    #[doc = "0: Do not enable DTCM power-on status event value."]
    NONE = 0,
    #[doc = "1: Enable GROUP0_DTCM0 power on status event value."]
    GROUP0DTCM0EN = 1,
    #[doc = "2: Enable GROUP0_DTCM1 power on status event value."]
    GROUP0DTCM1EN = 2,
    #[doc = "3: Enable DTCMs in group0 power on status event value."]
    GROUP0EN = 3,
    #[doc = "4: Enable DTCMs in group1 power on status event value."]
    GROUP1EN = 4,
    #[doc = "7: Enable all DTCM power on status event value."]
    ALL = 7,
}
impl From<DTCMEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DTCMEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTCMEN` reader - Enable DTCM power-on status event"]
pub struct DTCMEN_R(crate::FieldReader<u8, DTCMEN_A>);
impl DTCMEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTCMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTCMEN_A> {
        match self.bits {
            0 => Some(DTCMEN_A::NONE),
            1 => Some(DTCMEN_A::GROUP0DTCM0EN),
            2 => Some(DTCMEN_A::GROUP0DTCM1EN),
            3 => Some(DTCMEN_A::GROUP0EN),
            4 => Some(DTCMEN_A::GROUP1EN),
            7 => Some(DTCMEN_A::ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == DTCMEN_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0EN`"]
    #[inline(always)]
    pub fn is_group0dtcm0en(&self) -> bool {
        **self == DTCMEN_A::GROUP0DTCM0EN
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1EN`"]
    #[inline(always)]
    pub fn is_group0dtcm1en(&self) -> bool {
        **self == DTCMEN_A::GROUP0DTCM1EN
    }
    #[doc = "Checks if the value of the field is `GROUP0EN`"]
    #[inline(always)]
    pub fn is_group0en(&self) -> bool {
        **self == DTCMEN_A::GROUP0EN
    }
    #[doc = "Checks if the value of the field is `GROUP1EN`"]
    #[inline(always)]
    pub fn is_group1en(&self) -> bool {
        **self == DTCMEN_A::GROUP1EN
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == DTCMEN_A::ALL
    }
}
impl core::ops::Deref for DTCMEN_R {
    type Target = crate::FieldReader<u8, DTCMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCMEN` writer - Enable DTCM power-on status event"]
pub struct DTCMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCMEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not enable DTCM power-on status event value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCMEN_A::NONE)
    }
    #[doc = "Enable GROUP0_DTCM0 power on status event value."]
    #[inline(always)]
    pub fn group0dtcm0en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP0DTCM0EN)
    }
    #[doc = "Enable GROUP0_DTCM1 power on status event value."]
    #[inline(always)]
    pub fn group0dtcm1en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP0DTCM1EN)
    }
    #[doc = "Enable DTCMs in group0 power on status event value."]
    #[inline(always)]
    pub fn group0en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP0EN)
    }
    #[doc = "Enable DTCMs in group1 power on status event value."]
    #[inline(always)]
    pub fn group1en(self) -> &'a mut W {
        self.variant(DTCMEN_A::GROUP1EN)
    }
    #[doc = "Enable all DTCM power on status event value."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCMEN_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Control CACHEB2 power-on status event"]
    #[inline(always)]
    pub fn cacheb2en(&self) -> CACHEB2EN_R {
        CACHEB2EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Control CACHE BANK 0 power-on status event"]
    #[inline(always)]
    pub fn cacheb0en(&self) -> CACHEB0EN_R {
        CACHEB0EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash1en(&self) -> FLASH1EN_R {
        FLASH1EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash0en(&self) -> FLASH0EN_R {
        FLASH0EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 3:12 - Control SRAM power-on status event"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline(always)]
    pub fn dtcmen(&self) -> DTCMEN_R {
        DTCMEN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Control CACHEB2 power-on status event"]
    #[inline(always)]
    pub fn cacheb2en(&mut self) -> CACHEB2EN_W {
        CACHEB2EN_W { w: self }
    }
    #[doc = "Bit 30 - Control CACHE BANK 0 power-on status event"]
    #[inline(always)]
    pub fn cacheb0en(&mut self) -> CACHEB0EN_W {
        CACHEB0EN_W { w: self }
    }
    #[doc = "Bit 14 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash1en(&mut self) -> FLASH1EN_W {
        FLASH1EN_W { w: self }
    }
    #[doc = "Bit 13 - Control Flash power-on status event"]
    #[inline(always)]
    pub fn flash0en(&mut self) -> FLASH0EN_W {
        FLASH0EN_W { w: self }
    }
    #[doc = "Bits 3:12 - Control SRAM power-on status event"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W { w: self }
    }
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline(always)]
    pub fn dtcmen(&mut self) -> DTCMEN_W {
        DTCMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempwreventen](index.html) module"]
pub struct MEMPWREVENTEN_SPEC;
impl crate::RegisterSpec for MEMPWREVENTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mempwreventen::R](R) reader structure"]
impl crate::Readable for MEMPWREVENTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mempwreventen::W](W) writer structure"]
impl crate::Writable for MEMPWREVENTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMPWREVENTEN to value 0"]
impl crate::Resettable for MEMPWREVENTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

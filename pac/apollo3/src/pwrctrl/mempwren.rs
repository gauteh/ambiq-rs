#[doc = "Register `MEMPWREN` reader"]
pub struct R(crate::R<MEMPWREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMPWREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMPWREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMPWREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMPWREN` writer"]
pub struct W(crate::W<MEMPWREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMPWREN_SPEC>;
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
impl From<crate::W<MEMPWREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMPWREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB2_A {
    #[doc = "1: Power up Cache Bank 2 value."]
    EN = 1,
    #[doc = "0: Power down Cache Bank 2 value."]
    DIS = 0,
}
impl From<CACHEB2_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEB2` reader - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
pub struct CACHEB2_R(crate::FieldReader<bool, CACHEB2_A>);
impl CACHEB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEB2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB2_A {
        match self.bits {
            true => CACHEB2_A::EN,
            false => CACHEB2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CACHEB2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CACHEB2_A::DIS
    }
}
impl core::ops::Deref for CACHEB2_R {
    type Target = crate::FieldReader<bool, CACHEB2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEB2` writer - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
pub struct CACHEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up Cache Bank 2 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB2_A::EN)
    }
    #[doc = "Power down Cache Bank 2 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB2_A::DIS)
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
#[doc = "Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB0_A {
    #[doc = "1: Power up Cache Bank 0 value."]
    EN = 1,
    #[doc = "0: Power down Cache Bank 0 value."]
    DIS = 0,
}
impl From<CACHEB0_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEB0` reader - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
pub struct CACHEB0_R(crate::FieldReader<bool, CACHEB0_A>);
impl CACHEB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEB0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB0_A {
        match self.bits {
            true => CACHEB0_A::EN,
            false => CACHEB0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CACHEB0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CACHEB0_A::DIS
    }
}
impl core::ops::Deref for CACHEB0_R {
    type Target = crate::FieldReader<bool, CACHEB0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEB0` writer - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
pub struct CACHEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up Cache Bank 0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB0_A::EN)
    }
    #[doc = "Power down Cache Bank 0 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB0_A::DIS)
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
#[doc = "Power up Flash1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1_A {
    #[doc = "1: Power up Flash1 value."]
    EN = 1,
    #[doc = "0: Power down Flash1 value."]
    DIS = 0,
}
impl From<FLASH1_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH1` reader - Power up Flash1"]
pub struct FLASH1_R(crate::FieldReader<bool, FLASH1_A>);
impl FLASH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH1_A {
        match self.bits {
            true => FLASH1_A::EN,
            false => FLASH1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FLASH1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FLASH1_A::DIS
    }
}
impl core::ops::Deref for FLASH1_R {
    type Target = crate::FieldReader<bool, FLASH1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH1` writer - Power up Flash1"]
pub struct FLASH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up Flash1 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1_A::EN)
    }
    #[doc = "Power down Flash1 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1_A::DIS)
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
#[doc = "Power up Flash0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0_A {
    #[doc = "1: Power up Flash0 value."]
    EN = 1,
    #[doc = "0: Power down Flash0 value."]
    DIS = 0,
}
impl From<FLASH0_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH0` reader - Power up Flash0"]
pub struct FLASH0_R(crate::FieldReader<bool, FLASH0_A>);
impl FLASH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH0_A {
        match self.bits {
            true => FLASH0_A::EN,
            false => FLASH0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FLASH0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FLASH0_A::DIS
    }
}
impl core::ops::Deref for FLASH0_R {
    type Target = crate::FieldReader<bool, FLASH0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH0` writer - Power up Flash0"]
pub struct FLASH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up Flash0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0_A::EN)
    }
    #[doc = "Power down Flash0 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0_A::DIS)
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
#[doc = "Power up SRAM groups\n\nValue on reset: 1023"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAM_A {
    #[doc = "0: Do not power ON any of the SRAM banks value."]
    NONE = 0,
    #[doc = "1: Power ON only SRAM group0 (0KB-32KB) value."]
    GROUP0 = 1,
    #[doc = "2: Power ON only SRAM group1 (32KB-64KB) value."]
    GROUP1 = 2,
    #[doc = "4: Power ON only SRAM group2 (64KB-96KB) value."]
    GROUP2 = 4,
    #[doc = "8: Power ON only SRAM group3 (96KB-128KB) value."]
    GROUP3 = 8,
    #[doc = "16: Power ON only SRAM group4 (128KB-160KB) value."]
    GROUP4 = 16,
    #[doc = "32: Power ON only SRAM group5 (160KB-192KB) value."]
    GROUP5 = 32,
    #[doc = "64: Power ON only SRAM group6 (192KB-224KB) value."]
    GROUP6 = 64,
    #[doc = "128: Power ON only SRAM group7 (224KB-256KB) value."]
    GROUP7 = 128,
    #[doc = "256: Power ON only SRAM group8 (256KB-288KB) value."]
    GROUP8 = 256,
    #[doc = "512: Power ON only SRAM group9 (288KB-320KB) value."]
    GROUP9 = 512,
    #[doc = "3: Power ON only lower 64k value."]
    SRAM64K = 3,
    #[doc = "15: Power ON only lower 128k value."]
    SRAM128K = 15,
    #[doc = "255: Power ON only lower 256k value."]
    SRAM256K = 255,
    #[doc = "1023: All SRAM banks (320K) powered ON value."]
    ALL = 1023,
}
impl From<SRAM_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAM` reader - Power up SRAM groups"]
pub struct SRAM_R(crate::FieldReader<u16, SRAM_A>);
impl SRAM_R {
    pub(crate) fn new(bits: u16) -> Self {
        SRAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAM_A> {
        match self.bits {
            0 => Some(SRAM_A::NONE),
            1 => Some(SRAM_A::GROUP0),
            2 => Some(SRAM_A::GROUP1),
            4 => Some(SRAM_A::GROUP2),
            8 => Some(SRAM_A::GROUP3),
            16 => Some(SRAM_A::GROUP4),
            32 => Some(SRAM_A::GROUP5),
            64 => Some(SRAM_A::GROUP6),
            128 => Some(SRAM_A::GROUP7),
            256 => Some(SRAM_A::GROUP8),
            512 => Some(SRAM_A::GROUP9),
            3 => Some(SRAM_A::SRAM64K),
            15 => Some(SRAM_A::SRAM128K),
            255 => Some(SRAM_A::SRAM256K),
            1023 => Some(SRAM_A::ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SRAM_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        **self == SRAM_A::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        **self == SRAM_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `GROUP2`"]
    #[inline(always)]
    pub fn is_group2(&self) -> bool {
        **self == SRAM_A::GROUP2
    }
    #[doc = "Checks if the value of the field is `GROUP3`"]
    #[inline(always)]
    pub fn is_group3(&self) -> bool {
        **self == SRAM_A::GROUP3
    }
    #[doc = "Checks if the value of the field is `GROUP4`"]
    #[inline(always)]
    pub fn is_group4(&self) -> bool {
        **self == SRAM_A::GROUP4
    }
    #[doc = "Checks if the value of the field is `GROUP5`"]
    #[inline(always)]
    pub fn is_group5(&self) -> bool {
        **self == SRAM_A::GROUP5
    }
    #[doc = "Checks if the value of the field is `GROUP6`"]
    #[inline(always)]
    pub fn is_group6(&self) -> bool {
        **self == SRAM_A::GROUP6
    }
    #[doc = "Checks if the value of the field is `GROUP7`"]
    #[inline(always)]
    pub fn is_group7(&self) -> bool {
        **self == SRAM_A::GROUP7
    }
    #[doc = "Checks if the value of the field is `GROUP8`"]
    #[inline(always)]
    pub fn is_group8(&self) -> bool {
        **self == SRAM_A::GROUP8
    }
    #[doc = "Checks if the value of the field is `GROUP9`"]
    #[inline(always)]
    pub fn is_group9(&self) -> bool {
        **self == SRAM_A::GROUP9
    }
    #[doc = "Checks if the value of the field is `SRAM64K`"]
    #[inline(always)]
    pub fn is_sram64k(&self) -> bool {
        **self == SRAM_A::SRAM64K
    }
    #[doc = "Checks if the value of the field is `SRAM128K`"]
    #[inline(always)]
    pub fn is_sram128k(&self) -> bool {
        **self == SRAM_A::SRAM128K
    }
    #[doc = "Checks if the value of the field is `SRAM256K`"]
    #[inline(always)]
    pub fn is_sram256k(&self) -> bool {
        **self == SRAM_A::SRAM256K
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == SRAM_A::ALL
    }
}
impl core::ops::Deref for SRAM_R {
    type Target = crate::FieldReader<u16, SRAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM` writer - Power up SRAM groups"]
pub struct SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not power ON any of the SRAM banks value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAM_A::NONE)
    }
    #[doc = "Power ON only SRAM group0 (0KB-32KB) value."]
    #[inline(always)]
    pub fn group0(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP0)
    }
    #[doc = "Power ON only SRAM group1 (32KB-64KB) value."]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP1)
    }
    #[doc = "Power ON only SRAM group2 (64KB-96KB) value."]
    #[inline(always)]
    pub fn group2(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP2)
    }
    #[doc = "Power ON only SRAM group3 (96KB-128KB) value."]
    #[inline(always)]
    pub fn group3(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP3)
    }
    #[doc = "Power ON only SRAM group4 (128KB-160KB) value."]
    #[inline(always)]
    pub fn group4(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP4)
    }
    #[doc = "Power ON only SRAM group5 (160KB-192KB) value."]
    #[inline(always)]
    pub fn group5(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP5)
    }
    #[doc = "Power ON only SRAM group6 (192KB-224KB) value."]
    #[inline(always)]
    pub fn group6(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP6)
    }
    #[doc = "Power ON only SRAM group7 (224KB-256KB) value."]
    #[inline(always)]
    pub fn group7(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP7)
    }
    #[doc = "Power ON only SRAM group8 (256KB-288KB) value."]
    #[inline(always)]
    pub fn group8(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP8)
    }
    #[doc = "Power ON only SRAM group9 (288KB-320KB) value."]
    #[inline(always)]
    pub fn group9(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP9)
    }
    #[doc = "Power ON only lower 64k value."]
    #[inline(always)]
    pub fn sram64k(self) -> &'a mut W {
        self.variant(SRAM_A::SRAM64K)
    }
    #[doc = "Power ON only lower 128k value."]
    #[inline(always)]
    pub fn sram128k(self) -> &'a mut W {
        self.variant(SRAM_A::SRAM128K)
    }
    #[doc = "Power ON only lower 256k value."]
    #[inline(always)]
    pub fn sram256k(self) -> &'a mut W {
        self.variant(SRAM_A::SRAM256K)
    }
    #[doc = "All SRAM banks (320K) powered ON value."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAM_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 3)) | ((value as u32 & 0x03ff) << 3);
        self.w
    }
}
#[doc = "Power up DTCM\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTCM_A {
    #[doc = "0: Do not enable power to any DTCMs value."]
    NONE = 0,
    #[doc = "1: Power ON only GROUP0_DTCM0 value."]
    GROUP0DTCM0 = 1,
    #[doc = "2: Power ON only GROUP0_DTCM1 value."]
    GROUP0DTCM1 = 2,
    #[doc = "3: Power ON only DTCMs in group0 value."]
    GROUP0 = 3,
    #[doc = "4: Power ON only DTCMs in group1 value."]
    GROUP1 = 4,
    #[doc = "7: Power ON all DTCMs value."]
    ALL = 7,
}
impl From<DTCM_A> for u8 {
    #[inline(always)]
    fn from(variant: DTCM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTCM` reader - Power up DTCM"]
pub struct DTCM_R(crate::FieldReader<u8, DTCM_A>);
impl DTCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTCM_A> {
        match self.bits {
            0 => Some(DTCM_A::NONE),
            1 => Some(DTCM_A::GROUP0DTCM0),
            2 => Some(DTCM_A::GROUP0DTCM1),
            3 => Some(DTCM_A::GROUP0),
            4 => Some(DTCM_A::GROUP1),
            7 => Some(DTCM_A::ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == DTCM_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0`"]
    #[inline(always)]
    pub fn is_group0dtcm0(&self) -> bool {
        **self == DTCM_A::GROUP0DTCM0
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1`"]
    #[inline(always)]
    pub fn is_group0dtcm1(&self) -> bool {
        **self == DTCM_A::GROUP0DTCM1
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        **self == DTCM_A::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        **self == DTCM_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == DTCM_A::ALL
    }
}
impl core::ops::Deref for DTCM_R {
    type Target = crate::FieldReader<u8, DTCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCM` writer - Power up DTCM"]
pub struct DTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not enable power to any DTCMs value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCM_A::NONE)
    }
    #[doc = "Power ON only GROUP0_DTCM0 value."]
    #[inline(always)]
    pub fn group0dtcm0(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP0DTCM0)
    }
    #[doc = "Power ON only GROUP0_DTCM1 value."]
    #[inline(always)]
    pub fn group0dtcm1(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP0DTCM1)
    }
    #[doc = "Power ON only DTCMs in group0 value."]
    #[inline(always)]
    pub fn group0(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP0)
    }
    #[doc = "Power ON only DTCMs in group1 value."]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP1)
    }
    #[doc = "Power ON all DTCMs value."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCM_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb2(&self) -> CACHEB2_R {
        CACHEB2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb0(&self) -> CACHEB0_R {
        CACHEB0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Power up Flash1"]
    #[inline(always)]
    pub fn flash1(&self) -> FLASH1_R {
        FLASH1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Power up Flash0"]
    #[inline(always)]
    pub fn flash0(&self) -> FLASH0_R {
        FLASH0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 3:12 - Power up SRAM groups"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb2(&mut self) -> CACHEB2_W {
        CACHEB2_W { w: self }
    }
    #[doc = "Bit 30 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb0(&mut self) -> CACHEB0_W {
        CACHEB0_W { w: self }
    }
    #[doc = "Bit 14 - Power up Flash1"]
    #[inline(always)]
    pub fn flash1(&mut self) -> FLASH1_W {
        FLASH1_W { w: self }
    }
    #[doc = "Bit 13 - Power up Flash0"]
    #[inline(always)]
    pub fn flash0(&mut self) -> FLASH0_W {
        FLASH0_W { w: self }
    }
    #[doc = "Bits 3:12 - Power up SRAM groups"]
    #[inline(always)]
    pub fn sram(&mut self) -> SRAM_W {
        SRAM_W { w: self }
    }
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline(always)]
    pub fn dtcm(&mut self) -> DTCM_W {
        DTCM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enables individual banks of the MEMORY array\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempwren](index.html) module"]
pub struct MEMPWREN_SPEC;
impl crate::RegisterSpec for MEMPWREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mempwren::R](R) reader structure"]
impl crate::Readable for MEMPWREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mempwren::W](W) writer structure"]
impl crate::Writable for MEMPWREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMPWREN to value 0xc000_7fff"]
impl crate::Resettable for MEMPWREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_7fff
    }
}

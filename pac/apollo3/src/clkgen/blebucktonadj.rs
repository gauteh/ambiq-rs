#[doc = "Register `BLEBUCKTONADJ` reader"]
pub struct R(crate::R<BLEBUCKTONADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLEBUCKTONADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLEBUCKTONADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLEBUCKTONADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLEBUCKTONADJ` writer"]
pub struct W(crate::W<BLEBUCKTONADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLEBUCKTONADJ_SPEC>;
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
impl From<crate::W<BLEBUCKTONADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLEBUCKTONADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BLEBUCK ZERO LENGTH DETECT ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZEROLENDETECTEN_A {
    #[doc = "0: Disable Zero Length Detect value."]
    DIS = 0,
    #[doc = "1: Enable Zero Length Detect value."]
    EN = 1,
}
impl From<ZEROLENDETECTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ZEROLENDETECTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZEROLENDETECTEN` reader - BLEBUCK ZERO LENGTH DETECT ENABLE"]
pub struct ZEROLENDETECTEN_R(crate::FieldReader<bool, ZEROLENDETECTEN_A>);
impl ZEROLENDETECTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZEROLENDETECTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZEROLENDETECTEN_A {
        match self.bits {
            false => ZEROLENDETECTEN_A::DIS,
            true => ZEROLENDETECTEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ZEROLENDETECTEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ZEROLENDETECTEN_A::EN
    }
}
impl core::ops::Deref for ZEROLENDETECTEN_R {
    type Target = crate::FieldReader<bool, ZEROLENDETECTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZEROLENDETECTEN` writer - BLEBUCK ZERO LENGTH DETECT ENABLE"]
pub struct ZEROLENDETECTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROLENDETECTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZEROLENDETECTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Zero Length Detect value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ZEROLENDETECTEN_A::DIS)
    }
    #[doc = "Enable Zero Length Detect value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ZEROLENDETECTEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "BLEBUCK ZERO LENGTH DETECT TRIM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZEROLENDETECTTRIM_A {
    #[doc = "15: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 81us (10 percent margin of error) or more value."]
    SETF = 15,
    #[doc = "14: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 75.6us (10 percent margin of error) or more value."]
    SETE = 14,
    #[doc = "13: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 70.2us (10 percent margin of error) or more value."]
    SETD = 13,
    #[doc = "12: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 64.8us (10 percent margin of error) or more value."]
    SETC = 12,
    #[doc = "11: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 59.4us (10 percent margin of error) or more value."]
    SETB = 11,
    #[doc = "10: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 54.0us (10 percent margin of error) or more value."]
    SETA = 10,
    #[doc = "9: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 48.6us (10 percent margin of error) or more value."]
    SET9 = 9,
    #[doc = "8: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 43.2us (10 percent margin of error) or more value."]
    SET8 = 8,
    #[doc = "7: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 37.8us (10 percent margin of error) or more value."]
    SET7 = 7,
    #[doc = "6: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 32.4us (10 percent margin of error) or more value."]
    SET6 = 6,
    #[doc = "5: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 27.0us (10 percent margin of error) or more value."]
    SET5 = 5,
    #[doc = "4: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 21.6us (10 percent margin of error) or more value."]
    SET4 = 4,
    #[doc = "3: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 16.2us (10 percent margin of error) or more value."]
    SET3 = 3,
    #[doc = "2: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 10.8us (10 percent margin of error) or more value."]
    SET2 = 2,
    #[doc = "1: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 5.4us (10 percent margin of error) or more value."]
    SET1 = 1,
    #[doc = "0: Indicator send when the BLE BUCK asserts blebuck_comp1 for about 2.0us (10 percent margin of error) or more value."]
    SET0 = 0,
}
impl From<ZEROLENDETECTTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: ZEROLENDETECTTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZEROLENDETECTTRIM` reader - BLEBUCK ZERO LENGTH DETECT TRIM"]
pub struct ZEROLENDETECTTRIM_R(crate::FieldReader<u8, ZEROLENDETECTTRIM_A>);
impl ZEROLENDETECTTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZEROLENDETECTTRIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZEROLENDETECTTRIM_A {
        match self.bits {
            15 => ZEROLENDETECTTRIM_A::SETF,
            14 => ZEROLENDETECTTRIM_A::SETE,
            13 => ZEROLENDETECTTRIM_A::SETD,
            12 => ZEROLENDETECTTRIM_A::SETC,
            11 => ZEROLENDETECTTRIM_A::SETB,
            10 => ZEROLENDETECTTRIM_A::SETA,
            9 => ZEROLENDETECTTRIM_A::SET9,
            8 => ZEROLENDETECTTRIM_A::SET8,
            7 => ZEROLENDETECTTRIM_A::SET7,
            6 => ZEROLENDETECTTRIM_A::SET6,
            5 => ZEROLENDETECTTRIM_A::SET5,
            4 => ZEROLENDETECTTRIM_A::SET4,
            3 => ZEROLENDETECTTRIM_A::SET3,
            2 => ZEROLENDETECTTRIM_A::SET2,
            1 => ZEROLENDETECTTRIM_A::SET1,
            0 => ZEROLENDETECTTRIM_A::SET0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SETF`"]
    #[inline(always)]
    pub fn is_set_f(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SETF
    }
    #[doc = "Checks if the value of the field is `SETE`"]
    #[inline(always)]
    pub fn is_set_e(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SETE
    }
    #[doc = "Checks if the value of the field is `SETD`"]
    #[inline(always)]
    pub fn is_set_d(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SETD
    }
    #[doc = "Checks if the value of the field is `SETC`"]
    #[inline(always)]
    pub fn is_set_c(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SETC
    }
    #[doc = "Checks if the value of the field is `SETB`"]
    #[inline(always)]
    pub fn is_set_b(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SETB
    }
    #[doc = "Checks if the value of the field is `SETA`"]
    #[inline(always)]
    pub fn is_set_a(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SETA
    }
    #[doc = "Checks if the value of the field is `SET9`"]
    #[inline(always)]
    pub fn is_set9(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET9
    }
    #[doc = "Checks if the value of the field is `SET8`"]
    #[inline(always)]
    pub fn is_set8(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET8
    }
    #[doc = "Checks if the value of the field is `SET7`"]
    #[inline(always)]
    pub fn is_set7(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET7
    }
    #[doc = "Checks if the value of the field is `SET6`"]
    #[inline(always)]
    pub fn is_set6(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET6
    }
    #[doc = "Checks if the value of the field is `SET5`"]
    #[inline(always)]
    pub fn is_set5(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET5
    }
    #[doc = "Checks if the value of the field is `SET4`"]
    #[inline(always)]
    pub fn is_set4(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET4
    }
    #[doc = "Checks if the value of the field is `SET3`"]
    #[inline(always)]
    pub fn is_set3(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET3
    }
    #[doc = "Checks if the value of the field is `SET2`"]
    #[inline(always)]
    pub fn is_set2(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET2
    }
    #[doc = "Checks if the value of the field is `SET1`"]
    #[inline(always)]
    pub fn is_set1(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET1
    }
    #[doc = "Checks if the value of the field is `SET0`"]
    #[inline(always)]
    pub fn is_set0(&self) -> bool {
        **self == ZEROLENDETECTTRIM_A::SET0
    }
}
impl core::ops::Deref for ZEROLENDETECTTRIM_R {
    type Target = crate::FieldReader<u8, ZEROLENDETECTTRIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZEROLENDETECTTRIM` writer - BLEBUCK ZERO LENGTH DETECT TRIM"]
pub struct ZEROLENDETECTTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROLENDETECTTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZEROLENDETECTTRIM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 81us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set_f(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SETF)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 75.6us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set_e(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SETE)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 70.2us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set_d(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SETD)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 64.8us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set_c(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SETC)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 59.4us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set_b(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SETB)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 54.0us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set_a(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SETA)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 48.6us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set9(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET9)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 43.2us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set8(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET8)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 37.8us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set7(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET7)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 32.4us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set6(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET6)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 27.0us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set5(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET5)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 21.6us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set4(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET4)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 16.2us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set3(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET3)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 10.8us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set2(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET2)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 5.4us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set1(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET1)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 2.0us (10 percent margin of error) or more value."]
    #[inline(always)]
    pub fn set0(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIM_A::SET0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | ((value as u32 & 0x0f) << 23);
        self.w
    }
}
#[doc = "TON ADJUST ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TONADJUSTEN_A {
    #[doc = "0: Disable Adjust for BLE BUCK TON trim value."]
    DIS = 0,
    #[doc = "1: Enable Adjust for BLE BUCK TON trim value."]
    EN = 1,
}
impl From<TONADJUSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TONADJUSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TONADJUSTEN` reader - TON ADJUST ENABLE"]
pub struct TONADJUSTEN_R(crate::FieldReader<bool, TONADJUSTEN_A>);
impl TONADJUSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TONADJUSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TONADJUSTEN_A {
        match self.bits {
            false => TONADJUSTEN_A::DIS,
            true => TONADJUSTEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TONADJUSTEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TONADJUSTEN_A::EN
    }
}
impl core::ops::Deref for TONADJUSTEN_R {
    type Target = crate::FieldReader<bool, TONADJUSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TONADJUSTEN` writer - TON ADJUST ENABLE"]
pub struct TONADJUSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TONADJUSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TONADJUSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Adjust for BLE BUCK TON trim value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TONADJUSTEN_A::DIS)
    }
    #[doc = "Enable Adjust for BLE BUCK TON trim value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TONADJUSTEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "TON ADJUST PERIOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TONADJUSTPERIOD_A {
    #[doc = "3: Adjust done for every 1 3KHz period value."]
    HFRC_3KHZ = 3,
    #[doc = "2: Adjust done for every 1 12KHz period value."]
    HFRC_12KHZ = 2,
    #[doc = "1: Adjust done for every 1 47KHz period value."]
    HFRC_47KHZ = 1,
    #[doc = "0: Adjust done for every 1 94KHz period value."]
    HFRC_94KHZ = 0,
}
impl From<TONADJUSTPERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TONADJUSTPERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TONADJUSTPERIOD` reader - TON ADJUST PERIOD"]
pub struct TONADJUSTPERIOD_R(crate::FieldReader<u8, TONADJUSTPERIOD_A>);
impl TONADJUSTPERIOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TONADJUSTPERIOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TONADJUSTPERIOD_A {
        match self.bits {
            3 => TONADJUSTPERIOD_A::HFRC_3KHZ,
            2 => TONADJUSTPERIOD_A::HFRC_12KHZ,
            1 => TONADJUSTPERIOD_A::HFRC_47KHZ,
            0 => TONADJUSTPERIOD_A::HFRC_94KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRC_3KHZ`"]
    #[inline(always)]
    pub fn is_hfrc_3khz(&self) -> bool {
        **self == TONADJUSTPERIOD_A::HFRC_3KHZ
    }
    #[doc = "Checks if the value of the field is `HFRC_12KHZ`"]
    #[inline(always)]
    pub fn is_hfrc_12khz(&self) -> bool {
        **self == TONADJUSTPERIOD_A::HFRC_12KHZ
    }
    #[doc = "Checks if the value of the field is `HFRC_47KHZ`"]
    #[inline(always)]
    pub fn is_hfrc_47khz(&self) -> bool {
        **self == TONADJUSTPERIOD_A::HFRC_47KHZ
    }
    #[doc = "Checks if the value of the field is `HFRC_94KHZ`"]
    #[inline(always)]
    pub fn is_hfrc_94khz(&self) -> bool {
        **self == TONADJUSTPERIOD_A::HFRC_94KHZ
    }
}
impl core::ops::Deref for TONADJUSTPERIOD_R {
    type Target = crate::FieldReader<u8, TONADJUSTPERIOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TONADJUSTPERIOD` writer - TON ADJUST PERIOD"]
pub struct TONADJUSTPERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TONADJUSTPERIOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TONADJUSTPERIOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Adjust done for every 1 3KHz period value."]
    #[inline(always)]
    pub fn hfrc_3khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIOD_A::HFRC_3KHZ)
    }
    #[doc = "Adjust done for every 1 12KHz period value."]
    #[inline(always)]
    pub fn hfrc_12khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIOD_A::HFRC_12KHZ)
    }
    #[doc = "Adjust done for every 1 47KHz period value."]
    #[inline(always)]
    pub fn hfrc_47khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIOD_A::HFRC_47KHZ)
    }
    #[doc = "Adjust done for every 1 94KHz period value."]
    #[inline(always)]
    pub fn hfrc_94khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIOD_A::HFRC_94KHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `TONHIGHTHRESHOLD` reader - TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)"]
pub struct TONHIGHTHRESHOLD_R(crate::FieldReader<u16, u16>);
impl TONHIGHTHRESHOLD_R {
    pub(crate) fn new(bits: u16) -> Self {
        TONHIGHTHRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TONHIGHTHRESHOLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TONHIGHTHRESHOLD` writer - TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)"]
pub struct TONHIGHTHRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TONHIGHTHRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `TONLOWTHRESHOLD` reader - TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)"]
pub struct TONLOWTHRESHOLD_R(crate::FieldReader<u16, u16>);
impl TONLOWTHRESHOLD_R {
    pub(crate) fn new(bits: u16) -> Self {
        TONLOWTHRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TONLOWTHRESHOLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TONLOWTHRESHOLD` writer - TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)"]
pub struct TONLOWTHRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TONLOWTHRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - BLEBUCK ZERO LENGTH DETECT ENABLE"]
    #[inline(always)]
    pub fn zerolendetecten(&self) -> ZEROLENDETECTEN_R {
        ZEROLENDETECTEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - BLEBUCK ZERO LENGTH DETECT TRIM"]
    #[inline(always)]
    pub fn zerolendetecttrim(&self) -> ZEROLENDETECTTRIM_R {
        ZEROLENDETECTTRIM_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - TON ADJUST ENABLE"]
    #[inline(always)]
    pub fn tonadjusten(&self) -> TONADJUSTEN_R {
        TONADJUSTEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - TON ADJUST PERIOD"]
    #[inline(always)]
    pub fn tonadjustperiod(&self) -> TONADJUSTPERIOD_R {
        TONADJUSTPERIOD_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 10:19 - TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)"]
    #[inline(always)]
    pub fn tonhighthreshold(&self) -> TONHIGHTHRESHOLD_R {
        TONHIGHTHRESHOLD_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)"]
    #[inline(always)]
    pub fn tonlowthreshold(&self) -> TONLOWTHRESHOLD_R {
        TONLOWTHRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 27 - BLEBUCK ZERO LENGTH DETECT ENABLE"]
    #[inline(always)]
    pub fn zerolendetecten(&mut self) -> ZEROLENDETECTEN_W {
        ZEROLENDETECTEN_W { w: self }
    }
    #[doc = "Bits 23:26 - BLEBUCK ZERO LENGTH DETECT TRIM"]
    #[inline(always)]
    pub fn zerolendetecttrim(&mut self) -> ZEROLENDETECTTRIM_W {
        ZEROLENDETECTTRIM_W { w: self }
    }
    #[doc = "Bit 22 - TON ADJUST ENABLE"]
    #[inline(always)]
    pub fn tonadjusten(&mut self) -> TONADJUSTEN_W {
        TONADJUSTEN_W { w: self }
    }
    #[doc = "Bits 20:21 - TON ADJUST PERIOD"]
    #[inline(always)]
    pub fn tonadjustperiod(&mut self) -> TONADJUSTPERIOD_W {
        TONADJUSTPERIOD_W { w: self }
    }
    #[doc = "Bits 10:19 - TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)"]
    #[inline(always)]
    pub fn tonhighthreshold(&mut self) -> TONHIGHTHRESHOLD_W {
        TONHIGHTHRESHOLD_W { w: self }
    }
    #[doc = "Bits 0:9 - TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)"]
    #[inline(always)]
    pub fn tonlowthreshold(&mut self) -> TONLOWTHRESHOLD_W {
        TONLOWTHRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE BUCK TON ADJUST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blebucktonadj](index.html) module"]
pub struct BLEBUCKTONADJ_SPEC;
impl crate::RegisterSpec for BLEBUCKTONADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blebucktonadj::R](R) reader structure"]
impl crate::Readable for BLEBUCKTONADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blebucktonadj::W](W) writer structure"]
impl crate::Writable for BLEBUCKTONADJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLEBUCKTONADJ to value 0"]
impl crate::Resettable for BLEBUCKTONADJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

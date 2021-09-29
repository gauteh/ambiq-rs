#[doc = "Register `CHIPPN` reader"]
pub struct R(crate::R<CHIPPN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPPN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPPN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPPN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIPPN` writer"]
pub struct W(crate::W<CHIPPN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIPPN_SPEC>;
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
impl From<crate::W<CHIPPN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIPPN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BCD part number.\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PARTNUM_A {
    #[doc = "100663296: Apollo3 part number is 0x06xxxxxx. value."]
    APOLLO3 = 100663296,
    #[doc = "50331648: Apollo2 part number is 0x03xxxxxx. value."]
    APOLLO2 = 50331648,
    #[doc = "16777216: Apollo part number is 0x01xxxxxx. value."]
    APOLLO = 16777216,
    #[doc = "4278190080: Mask for the part number field. value."]
    PN_M = 4278190080,
    #[doc = "24: Bit position for the part number field. value."]
    PN_S = 24,
    #[doc = "15728640: Mask for the FLASH_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 2MB value."]
    FLASHSIZE_M = 15728640,
    #[doc = "20: Bit position for the FLASH_SIZE field. value."]
    FLASHSIZE_S = 20,
    #[doc = "983040: Mask for the SRAM_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 384KB value."]
    SRAMSIZE_M = 983040,
    #[doc = "16: Bit position for the SRAM_SIZE field. value."]
    SRAMSIZE_S = 16,
    #[doc = "65280: Mask for the revision field. Bits \\[15:12\\]
are major rev, \\[11:8\\]
are minor rev.\nValues:\n0: Major Rev A, Minor Rev 0\n1: Major Rev B, Minor Rev 1 value."]
    REV_M = 65280,
    #[doc = "8: Bit position for the revision field. value."]
    REV_S = 8,
    #[doc = "192: Mask for the package field.\nValues:\n0: SIP\n1: QFN\n2: BGA\n3: CSP value."]
    PKG_M = 192,
    #[doc = "6: Bit position for the package field. value."]
    PKG_S = 6,
    #[doc = "56: Mask for the pins field.\nValues:\n0: 25 pins\n1: 49 pins\n2: 64 pins\n3: 81 pins value."]
    PINS_M = 56,
    #[doc = "3: Bit position for the pins field. value."]
    PINS_S = 3,
    #[doc = "1: Bit position for the temperature field. value."]
    TEMP_S = 1,
    #[doc = "0: Bit position for the qualified field. value."]
    QUAL_S = 0,
}
impl From<PARTNUM_A> for u32 {
    #[inline(always)]
    fn from(variant: PARTNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PARTNUM` reader - BCD part number."]
pub struct PARTNUM_R(crate::FieldReader<u32, PARTNUM_A>);
impl PARTNUM_R {
    pub(crate) fn new(bits: u32) -> Self {
        PARTNUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARTNUM_A> {
        match self.bits {
            100663296 => Some(PARTNUM_A::APOLLO3),
            50331648 => Some(PARTNUM_A::APOLLO2),
            16777216 => Some(PARTNUM_A::APOLLO),
            4278190080 => Some(PARTNUM_A::PN_M),
            24 => Some(PARTNUM_A::PN_S),
            15728640 => Some(PARTNUM_A::FLASHSIZE_M),
            20 => Some(PARTNUM_A::FLASHSIZE_S),
            983040 => Some(PARTNUM_A::SRAMSIZE_M),
            16 => Some(PARTNUM_A::SRAMSIZE_S),
            65280 => Some(PARTNUM_A::REV_M),
            8 => Some(PARTNUM_A::REV_S),
            192 => Some(PARTNUM_A::PKG_M),
            6 => Some(PARTNUM_A::PKG_S),
            56 => Some(PARTNUM_A::PINS_M),
            3 => Some(PARTNUM_A::PINS_S),
            1 => Some(PARTNUM_A::TEMP_S),
            0 => Some(PARTNUM_A::QUAL_S),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO3`"]
    #[inline(always)]
    pub fn is_apollo3(&self) -> bool {
        **self == PARTNUM_A::APOLLO3
    }
    #[doc = "Checks if the value of the field is `APOLLO2`"]
    #[inline(always)]
    pub fn is_apollo2(&self) -> bool {
        **self == PARTNUM_A::APOLLO2
    }
    #[doc = "Checks if the value of the field is `APOLLO`"]
    #[inline(always)]
    pub fn is_apollo(&self) -> bool {
        **self == PARTNUM_A::APOLLO
    }
    #[doc = "Checks if the value of the field is `PN_M`"]
    #[inline(always)]
    pub fn is_pn_m(&self) -> bool {
        **self == PARTNUM_A::PN_M
    }
    #[doc = "Checks if the value of the field is `PN_S`"]
    #[inline(always)]
    pub fn is_pn_s(&self) -> bool {
        **self == PARTNUM_A::PN_S
    }
    #[doc = "Checks if the value of the field is `FLASHSIZE_M`"]
    #[inline(always)]
    pub fn is_flashsize_m(&self) -> bool {
        **self == PARTNUM_A::FLASHSIZE_M
    }
    #[doc = "Checks if the value of the field is `FLASHSIZE_S`"]
    #[inline(always)]
    pub fn is_flashsize_s(&self) -> bool {
        **self == PARTNUM_A::FLASHSIZE_S
    }
    #[doc = "Checks if the value of the field is `SRAMSIZE_M`"]
    #[inline(always)]
    pub fn is_sramsize_m(&self) -> bool {
        **self == PARTNUM_A::SRAMSIZE_M
    }
    #[doc = "Checks if the value of the field is `SRAMSIZE_S`"]
    #[inline(always)]
    pub fn is_sramsize_s(&self) -> bool {
        **self == PARTNUM_A::SRAMSIZE_S
    }
    #[doc = "Checks if the value of the field is `REV_M`"]
    #[inline(always)]
    pub fn is_rev_m(&self) -> bool {
        **self == PARTNUM_A::REV_M
    }
    #[doc = "Checks if the value of the field is `REV_S`"]
    #[inline(always)]
    pub fn is_rev_s(&self) -> bool {
        **self == PARTNUM_A::REV_S
    }
    #[doc = "Checks if the value of the field is `PKG_M`"]
    #[inline(always)]
    pub fn is_pkg_m(&self) -> bool {
        **self == PARTNUM_A::PKG_M
    }
    #[doc = "Checks if the value of the field is `PKG_S`"]
    #[inline(always)]
    pub fn is_pkg_s(&self) -> bool {
        **self == PARTNUM_A::PKG_S
    }
    #[doc = "Checks if the value of the field is `PINS_M`"]
    #[inline(always)]
    pub fn is_pins_m(&self) -> bool {
        **self == PARTNUM_A::PINS_M
    }
    #[doc = "Checks if the value of the field is `PINS_S`"]
    #[inline(always)]
    pub fn is_pins_s(&self) -> bool {
        **self == PARTNUM_A::PINS_S
    }
    #[doc = "Checks if the value of the field is `TEMP_S`"]
    #[inline(always)]
    pub fn is_temp_s(&self) -> bool {
        **self == PARTNUM_A::TEMP_S
    }
    #[doc = "Checks if the value of the field is `QUAL_S`"]
    #[inline(always)]
    pub fn is_qual_s(&self) -> bool {
        **self == PARTNUM_A::QUAL_S
    }
}
impl core::ops::Deref for PARTNUM_R {
    type Target = crate::FieldReader<u32, PARTNUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTNUM` writer - BCD part number."]
pub struct PARTNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTNUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARTNUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 part number is 0x06xxxxxx. value."]
    #[inline(always)]
    pub fn apollo3(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO3)
    }
    #[doc = "Apollo2 part number is 0x03xxxxxx. value."]
    #[inline(always)]
    pub fn apollo2(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO2)
    }
    #[doc = "Apollo part number is 0x01xxxxxx. value."]
    #[inline(always)]
    pub fn apollo(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO)
    }
    #[doc = "Mask for the part number field. value."]
    #[inline(always)]
    pub fn pn_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::PN_M)
    }
    #[doc = "Bit position for the part number field. value."]
    #[inline(always)]
    pub fn pn_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::PN_S)
    }
    #[doc = "Mask for the FLASH_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 2MB value."]
    #[inline(always)]
    pub fn flashsize_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::FLASHSIZE_M)
    }
    #[doc = "Bit position for the FLASH_SIZE field. value."]
    #[inline(always)]
    pub fn flashsize_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::FLASHSIZE_S)
    }
    #[doc = "Mask for the SRAM_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 384KB value."]
    #[inline(always)]
    pub fn sramsize_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::SRAMSIZE_M)
    }
    #[doc = "Bit position for the SRAM_SIZE field. value."]
    #[inline(always)]
    pub fn sramsize_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::SRAMSIZE_S)
    }
    #[doc = "Mask for the revision field. Bits \\[15:12\\]
are major rev, \\[11:8\\]
are minor rev. Values: 0: Major Rev A, Minor Rev 0 1: Major Rev B, Minor Rev 1 value."]
    #[inline(always)]
    pub fn rev_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::REV_M)
    }
    #[doc = "Bit position for the revision field. value."]
    #[inline(always)]
    pub fn rev_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::REV_S)
    }
    #[doc = "Mask for the package field. Values: 0: SIP 1: QFN 2: BGA 3: CSP value."]
    #[inline(always)]
    pub fn pkg_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::PKG_M)
    }
    #[doc = "Bit position for the package field. value."]
    #[inline(always)]
    pub fn pkg_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::PKG_S)
    }
    #[doc = "Mask for the pins field. Values: 0: 25 pins 1: 49 pins 2: 64 pins 3: 81 pins value."]
    #[inline(always)]
    pub fn pins_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::PINS_M)
    }
    #[doc = "Bit position for the pins field. value."]
    #[inline(always)]
    pub fn pins_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::PINS_S)
    }
    #[doc = "Bit position for the temperature field. value."]
    #[inline(always)]
    pub fn temp_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::TEMP_S)
    }
    #[doc = "Bit position for the qualified field. value."]
    #[inline(always)]
    pub fn qual_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::QUAL_S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline(always)]
    pub fn partnum(&mut self) -> PARTNUM_W {
        PARTNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Information Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chippn](index.html) module"]
pub struct CHIPPN_SPEC;
impl crate::RegisterSpec for CHIPPN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chippn::R](R) reader structure"]
impl crate::Readable for CHIPPN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chippn::W](W) writer structure"]
impl crate::Writable for CHIPPN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHIPPN to value 0x0400_0000"]
impl crate::Resettable for CHIPPN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}

#[doc = "Register `STCFG` reader"]
pub struct R(crate::R<STCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCFG` writer"]
pub struct W(crate::W<STCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCFG_SPEC>;
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
impl From<crate::W<STCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREEZE_A {
    #[doc = "0: Let the COUNTER register run on its input clock. value."]
    THAW = 0,
    #[doc = "1: Stop the COUNTER register for loading. value."]
    FREEZE = 1,
}
impl From<FREEZE_A> for bool {
    #[inline(always)]
    fn from(variant: FREEZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREEZE` reader - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
pub struct FREEZE_R(crate::FieldReader<bool, FREEZE_A>);
impl FREEZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREEZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREEZE_A {
        match self.bits {
            false => FREEZE_A::THAW,
            true => FREEZE_A::FREEZE,
        }
    }
    #[doc = "Checks if the value of the field is `THAW`"]
    #[inline(always)]
    pub fn is_thaw(&self) -> bool {
        **self == FREEZE_A::THAW
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        **self == FREEZE_A::FREEZE
    }
}
impl core::ops::Deref for FREEZE_R {
    type Target = crate::FieldReader<bool, FREEZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREEZE` writer - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
pub struct FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREEZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREEZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Let the COUNTER register run on its input clock. value."]
    #[inline(always)]
    pub fn thaw(self) -> &'a mut W {
        self.variant(FREEZE_A::THAW)
    }
    #[doc = "Stop the COUNTER register for loading. value."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut W {
        self.variant(FREEZE_A::FREEZE)
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
#[doc = "Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEAR_A {
    #[doc = "0: Let the COUNTER register run on its input clock. value."]
    RUN = 0,
    #[doc = "1: Stop the COUNTER register for loading. value."]
    CLEAR = 1,
}
impl From<CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEAR` reader - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
pub struct CLEAR_R(crate::FieldReader<bool, CLEAR_A>);
impl CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLEAR_A {
        match self.bits {
            false => CLEAR_A::RUN,
            true => CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == CLEAR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for CLEAR_R {
    type Target = crate::FieldReader<bool, CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAR` writer - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Let the COUNTER register run on its input clock. value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLEAR_A::RUN)
    }
    #[doc = "Stop the COUNTER register for loading. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEAR_A::CLEAR)
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
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_H_EN_A {
    #[doc = "0: Compare H disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare H enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_H_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_H_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_H_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_H_EN_R(crate::FieldReader<bool, COMPARE_H_EN_A>);
impl COMPARE_H_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_H_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_H_EN_A {
        match self.bits {
            false => COMPARE_H_EN_A::DISABLE,
            true => COMPARE_H_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_H_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_H_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_H_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_H_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_H_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_H_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_H_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_H_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare H disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_H_EN_A::DISABLE)
    }
    #[doc = "Compare H enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_H_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_G_EN_A {
    #[doc = "0: Compare G disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare G enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_G_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_G_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_G_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_G_EN_R(crate::FieldReader<bool, COMPARE_G_EN_A>);
impl COMPARE_G_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_G_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_G_EN_A {
        match self.bits {
            false => COMPARE_G_EN_A::DISABLE,
            true => COMPARE_G_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_G_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_G_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_G_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_G_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_G_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_G_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_G_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_G_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare G disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_G_EN_A::DISABLE)
    }
    #[doc = "Compare G enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_G_EN_A::ENABLE)
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
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_F_EN_A {
    #[doc = "0: Compare F disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare F enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_F_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_F_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_F_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_F_EN_R(crate::FieldReader<bool, COMPARE_F_EN_A>);
impl COMPARE_F_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_F_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_F_EN_A {
        match self.bits {
            false => COMPARE_F_EN_A::DISABLE,
            true => COMPARE_F_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_F_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_F_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_F_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_F_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_F_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_F_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_F_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_F_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare F disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_F_EN_A::DISABLE)
    }
    #[doc = "Compare F enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_F_EN_A::ENABLE)
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
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_E_EN_A {
    #[doc = "0: Compare E disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare E enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_E_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_E_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_E_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_E_EN_R(crate::FieldReader<bool, COMPARE_E_EN_A>);
impl COMPARE_E_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_E_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_E_EN_A {
        match self.bits {
            false => COMPARE_E_EN_A::DISABLE,
            true => COMPARE_E_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_E_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_E_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_E_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_E_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_E_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_E_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_E_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_E_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare E disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_E_EN_A::DISABLE)
    }
    #[doc = "Compare E enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_E_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_D_EN_A {
    #[doc = "0: Compare D disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare D enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_D_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_D_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_D_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_D_EN_R(crate::FieldReader<bool, COMPARE_D_EN_A>);
impl COMPARE_D_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_D_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_D_EN_A {
        match self.bits {
            false => COMPARE_D_EN_A::DISABLE,
            true => COMPARE_D_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_D_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_D_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_D_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_D_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_D_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_D_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_D_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_D_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare D disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_D_EN_A::DISABLE)
    }
    #[doc = "Compare D enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_D_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_C_EN_A {
    #[doc = "0: Compare C disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare C enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_C_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_C_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_C_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_C_EN_R(crate::FieldReader<bool, COMPARE_C_EN_A>);
impl COMPARE_C_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_C_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_C_EN_A {
        match self.bits {
            false => COMPARE_C_EN_A::DISABLE,
            true => COMPARE_C_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_C_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_C_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_C_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_C_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_C_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_C_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_C_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_C_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare C disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_C_EN_A::DISABLE)
    }
    #[doc = "Compare C enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_C_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_B_EN_A {
    #[doc = "0: Compare B disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare B enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_B_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_B_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_B_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_B_EN_R(crate::FieldReader<bool, COMPARE_B_EN_A>);
impl COMPARE_B_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_B_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_B_EN_A {
        match self.bits {
            false => COMPARE_B_EN_A::DISABLE,
            true => COMPARE_B_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_B_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_B_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_B_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_B_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_B_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_B_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_B_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_B_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare B disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_B_EN_A::DISABLE)
    }
    #[doc = "Compare B enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_B_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_A_EN_A {
    #[doc = "0: Compare A disabled. value."]
    DISABLE = 0,
    #[doc = "1: Compare A enabled. value."]
    ENABLE = 1,
}
impl From<COMPARE_A_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_A_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE_A_EN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_A_EN_R(crate::FieldReader<bool, COMPARE_A_EN_A>);
impl COMPARE_A_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_A_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_A_EN_A {
        match self.bits {
            false => COMPARE_A_EN_A::DISABLE,
            true => COMPARE_A_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMPARE_A_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMPARE_A_EN_A::ENABLE
    }
}
impl core::ops::Deref for COMPARE_A_EN_R {
    type Target = crate::FieldReader<bool, COMPARE_A_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE_A_EN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub struct COMPARE_A_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_A_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_A_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare A disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_A_EN_A::DISABLE)
    }
    #[doc = "Compare A enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_A_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Selects an appropriate clock source and divider to use for the System Timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: No clock enabled. value."]
    NOCLK = 0,
    #[doc = "1: 3MHz from the HFRC clock divider. value."]
    HFRC_DIV16 = 1,
    #[doc = "2: 187.5KHz from the HFRC clock divider. value."]
    HFRC_DIV256 = 2,
    #[doc = "3: 32768Hz from the crystal oscillator. value."]
    XTAL_DIV1 = 3,
    #[doc = "4: 16384Hz from the crystal oscillator. value."]
    XTAL_DIV2 = 4,
    #[doc = "5: 1024Hz from the crystal oscillator. value."]
    XTAL_DIV32 = 5,
    #[doc = "6: Approximately 1KHz from the LFRC oscillator (uncalibrated). value."]
    LFRC_DIV1 = 6,
    #[doc = "7: Use CTIMER 0 section A as a prescaler for the clock source. value."]
    CTIMER0A = 7,
    #[doc = "8: Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source. value."]
    CTIMER0B = 8,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKSEL` reader - Selects an appropriate clock source and divider to use for the System Timer clock."]
pub struct CLKSEL_R(crate::FieldReader<u8, CLKSEL_A>);
impl CLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::NOCLK),
            1 => Some(CLKSEL_A::HFRC_DIV16),
            2 => Some(CLKSEL_A::HFRC_DIV256),
            3 => Some(CLKSEL_A::XTAL_DIV1),
            4 => Some(CLKSEL_A::XTAL_DIV2),
            5 => Some(CLKSEL_A::XTAL_DIV32),
            6 => Some(CLKSEL_A::LFRC_DIV1),
            7 => Some(CLKSEL_A::CTIMER0A),
            8 => Some(CLKSEL_A::CTIMER0B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOCLK`"]
    #[inline(always)]
    pub fn is_noclk(&self) -> bool {
        **self == CLKSEL_A::NOCLK
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == CLKSEL_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == CLKSEL_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV1`"]
    #[inline(always)]
    pub fn is_xtal_div1(&self) -> bool {
        **self == CLKSEL_A::XTAL_DIV1
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV2`"]
    #[inline(always)]
    pub fn is_xtal_div2(&self) -> bool {
        **self == CLKSEL_A::XTAL_DIV2
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV32`"]
    #[inline(always)]
    pub fn is_xtal_div32(&self) -> bool {
        **self == CLKSEL_A::XTAL_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1`"]
    #[inline(always)]
    pub fn is_lfrc_div1(&self) -> bool {
        **self == CLKSEL_A::LFRC_DIV1
    }
    #[doc = "Checks if the value of the field is `CTIMER0A`"]
    #[inline(always)]
    pub fn is_ctimer0a(&self) -> bool {
        **self == CLKSEL_A::CTIMER0A
    }
    #[doc = "Checks if the value of the field is `CTIMER0B`"]
    #[inline(always)]
    pub fn is_ctimer0b(&self) -> bool {
        **self == CLKSEL_A::CTIMER0B
    }
}
impl core::ops::Deref for CLKSEL_R {
    type Target = crate::FieldReader<u8, CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL` writer - Selects an appropriate clock source and divider to use for the System Timer clock."]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock enabled. value."]
    #[inline(always)]
    pub fn noclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::NOCLK)
    }
    #[doc = "3MHz from the HFRC clock divider. value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRC_DIV16)
    }
    #[doc = "187.5KHz from the HFRC clock divider. value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRC_DIV256)
    }
    #[doc = "32768Hz from the crystal oscillator. value."]
    #[inline(always)]
    pub fn xtal_div1(self) -> &'a mut W {
        self.variant(CLKSEL_A::XTAL_DIV1)
    }
    #[doc = "16384Hz from the crystal oscillator. value."]
    #[inline(always)]
    pub fn xtal_div2(self) -> &'a mut W {
        self.variant(CLKSEL_A::XTAL_DIV2)
    }
    #[doc = "1024Hz from the crystal oscillator. value."]
    #[inline(always)]
    pub fn xtal_div32(self) -> &'a mut W {
        self.variant(CLKSEL_A::XTAL_DIV32)
    }
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated). value."]
    #[inline(always)]
    pub fn lfrc_div1(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFRC_DIV1)
    }
    #[doc = "Use CTIMER 0 section A as a prescaler for the clock source. value."]
    #[inline(always)]
    pub fn ctimer0a(self) -> &'a mut W {
        self.variant(CLKSEL_A::CTIMER0A)
    }
    #[doc = "Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source. value."]
    #[inline(always)]
    pub fn ctimer0b(self) -> &'a mut W {
        self.variant(CLKSEL_A::CTIMER0B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_h_en(&self) -> COMPARE_H_EN_R {
        COMPARE_H_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_g_en(&self) -> COMPARE_G_EN_R {
        COMPARE_G_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_f_en(&self) -> COMPARE_F_EN_R {
        COMPARE_F_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_e_en(&self) -> COMPARE_E_EN_R {
        COMPARE_E_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_d_en(&self) -> COMPARE_D_EN_R {
        COMPARE_D_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_c_en(&self) -> COMPARE_C_EN_R {
        COMPARE_C_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_b_en(&self) -> COMPARE_B_EN_R {
        COMPARE_B_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_a_en(&self) -> COMPARE_A_EN_R {
        COMPARE_A_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline(always)]
    pub fn freeze(&mut self) -> FREEZE_W {
        FREEZE_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_h_en(&mut self) -> COMPARE_H_EN_W {
        COMPARE_H_EN_W { w: self }
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_g_en(&mut self) -> COMPARE_G_EN_W {
        COMPARE_G_EN_W { w: self }
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_f_en(&mut self) -> COMPARE_F_EN_W {
        COMPARE_F_EN_W { w: self }
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_e_en(&mut self) -> COMPARE_E_EN_W {
        COMPARE_E_EN_W { w: self }
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_d_en(&mut self) -> COMPARE_D_EN_W {
        COMPARE_D_EN_W { w: self }
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_c_en(&mut self) -> COMPARE_C_EN_W {
        COMPARE_C_EN_W { w: self }
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_b_en(&mut self) -> COMPARE_B_EN_W {
        COMPARE_B_EN_W { w: self }
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_a_en(&mut self) -> COMPARE_A_EN_W {
        COMPARE_A_EN_W { w: self }
    }
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcfg](index.html) module"]
pub struct STCFG_SPEC;
impl crate::RegisterSpec for STCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcfg::R](R) reader structure"]
impl crate::Readable for STCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcfg::W](W) writer structure"]
impl crate::Writable for STCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STCFG to value 0x8000_0000"]
impl crate::Resettable for STCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}

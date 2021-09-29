#[doc = "Register `CACHECFG` reader"]
pub struct R(crate::R<CACHECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHECFG` writer"]
pub struct W(crate::W<CACHECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHECFG_SPEC>;
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
impl From<crate::W<CACHECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_MONITOR` reader - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
pub struct ENABLE_MONITOR_R(crate::FieldReader<bool, bool>);
impl ENABLE_MONITOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_MONITOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_MONITOR` writer - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
pub struct ENABLE_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_MONITOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `DATA_CLKGATE` reader - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
pub struct DATA_CLKGATE_R(crate::FieldReader<bool, bool>);
impl DATA_CLKGATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_CLKGATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_CLKGATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_CLKGATE` writer - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
pub struct DATA_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CACHE_LS` reader - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
pub struct CACHE_LS_R(crate::FieldReader<bool, bool>);
impl CACHE_LS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_LS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_LS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_LS` writer - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
pub struct CACHE_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_LS_W<'a> {
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
#[doc = "Field `CACHE_CLKGATE` reader - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
pub struct CACHE_CLKGATE_R(crate::FieldReader<bool, bool>);
impl CACHE_CLKGATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_CLKGATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_CLKGATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_CLKGATE` writer - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
pub struct CACHE_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_CLKGATE_W<'a> {
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
#[doc = "Field `DCACHE_ENABLE` reader - Enable Flash Data Caching."]
pub struct DCACHE_ENABLE_R(crate::FieldReader<bool, bool>);
impl DCACHE_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_ENABLE` writer - Enable Flash Data Caching."]
pub struct DCACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_ENABLE_W<'a> {
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
#[doc = "Field `ICACHE_ENABLE` reader - Enable Flash Instruction Caching"]
pub struct ICACHE_ENABLE_R(crate::FieldReader<bool, bool>);
impl ICACHE_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_ENABLE` writer - Enable Flash Instruction Caching"]
pub struct ICACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_ENABLE_W<'a> {
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
#[doc = "Sets the cache configuration\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONFIG_A {
    #[doc = "4: Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    W1_128B_512E = 4,
    #[doc = "5: Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    W2_128B_512E = 5,
    #[doc = "8: Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    W1_128B_1024E = 8,
}
impl From<CONFIG_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CONFIG` reader - Sets the cache configuration"]
pub struct CONFIG_R(crate::FieldReader<u8, CONFIG_A>);
impl CONFIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CONFIG_A> {
        match self.bits {
            4 => Some(CONFIG_A::W1_128B_512E),
            5 => Some(CONFIG_A::W2_128B_512E),
            8 => Some(CONFIG_A::W1_128B_1024E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `W1_128B_512E`"]
    #[inline(always)]
    pub fn is_w1_128b_512e(&self) -> bool {
        **self == CONFIG_A::W1_128B_512E
    }
    #[doc = "Checks if the value of the field is `W2_128B_512E`"]
    #[inline(always)]
    pub fn is_w2_128b_512e(&self) -> bool {
        **self == CONFIG_A::W2_128B_512E
    }
    #[doc = "Checks if the value of the field is `W1_128B_1024E`"]
    #[inline(always)]
    pub fn is_w1_128b_1024e(&self) -> bool {
        **self == CONFIG_A::W1_128B_1024E
    }
}
impl core::ops::Deref for CONFIG_R {
    type Target = crate::FieldReader<u8, CONFIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFIG` writer - Sets the cache configuration"]
pub struct CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONFIG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    #[inline(always)]
    pub fn w1_128b_512e(self) -> &'a mut W {
        self.variant(CONFIG_A::W1_128B_512E)
    }
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    #[inline(always)]
    pub fn w2_128b_512e(self) -> &'a mut W {
        self.variant(CONFIG_A::W2_128B_512E)
    }
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    #[inline(always)]
    pub fn w1_128b_1024e(self) -> &'a mut W {
        self.variant(CONFIG_A::W1_128B_1024E)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ENABLE_NC1` reader - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
pub struct ENABLE_NC1_R(crate::FieldReader<bool, bool>);
impl ENABLE_NC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_NC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_NC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_NC1` writer - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
pub struct ENABLE_NC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_NC1_W<'a> {
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
#[doc = "Field `ENABLE_NC0` reader - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
pub struct ENABLE_NC0_R(crate::FieldReader<bool, bool>);
impl ENABLE_NC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_NC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_NC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_NC0` writer - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
pub struct ENABLE_NC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_NC0_W<'a> {
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
#[doc = "Field `LRU` reader - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
pub struct LRU_R(crate::FieldReader<bool, bool>);
impl LRU_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LRU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRU` writer - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
pub struct LRU_W<'a> {
    w: &'a mut W,
}
impl<'a> LRU_W<'a> {
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
#[doc = "Field `ENABLE` reader - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline(always)]
    pub fn enable_monitor(&self) -> ENABLE_MONITOR_R {
        ENABLE_MONITOR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline(always)]
    pub fn data_clkgate(&self) -> DATA_CLKGATE_R {
        DATA_CLKGATE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline(always)]
    pub fn cache_ls(&self) -> CACHE_LS_R {
        CACHE_LS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline(always)]
    pub fn cache_clkgate(&self) -> CACHE_CLKGATE_R {
        CACHE_CLKGATE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Flash Data Caching."]
    #[inline(always)]
    pub fn dcache_enable(&self) -> DCACHE_ENABLE_R {
        DCACHE_ENABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching"]
    #[inline(always)]
    pub fn icache_enable(&self) -> ICACHE_ENABLE_R {
        ICACHE_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc1(&self) -> ENABLE_NC1_R {
        ENABLE_NC1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc0(&self) -> ENABLE_NC0_R {
        ENABLE_NC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline(always)]
    pub fn enable_monitor(&mut self) -> ENABLE_MONITOR_W {
        ENABLE_MONITOR_W { w: self }
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline(always)]
    pub fn data_clkgate(&mut self) -> DATA_CLKGATE_W {
        DATA_CLKGATE_W { w: self }
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline(always)]
    pub fn cache_ls(&mut self) -> CACHE_LS_W {
        CACHE_LS_W { w: self }
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline(always)]
    pub fn cache_clkgate(&mut self) -> CACHE_CLKGATE_W {
        CACHE_CLKGATE_W { w: self }
    }
    #[doc = "Bit 9 - Enable Flash Data Caching."]
    #[inline(always)]
    pub fn dcache_enable(&mut self) -> DCACHE_ENABLE_W {
        DCACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching"]
    #[inline(always)]
    pub fn icache_enable(&mut self) -> ICACHE_ENABLE_W {
        ICACHE_ENABLE_W { w: self }
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W {
        CONFIG_W { w: self }
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc1(&mut self) -> ENABLE_NC1_W {
        ENABLE_NC1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline(always)]
    pub fn enable_nc0(&mut self) -> ENABLE_NC0_W {
        ENABLE_NC0_W { w: self }
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline(always)]
    pub fn lru(&mut self) -> LRU_W {
        LRU_W { w: self }
    }
    #[doc = "Bit 0 - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Cache Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachecfg](index.html) module"]
pub struct CACHECFG_SPEC;
impl crate::RegisterSpec for CACHECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cachecfg::R](R) reader structure"]
impl crate::Readable for CACHECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cachecfg::W](W) writer structure"]
impl crate::Writable for CACHECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHECFG to value 0x0010_0c50"]
impl crate::Resettable for CACHECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0c50
    }
}

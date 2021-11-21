#[doc = "Register `VLAN_HASHTABLE` reader"]
pub struct R(crate::R<VLAN_HASHTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLAN_HASHTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLAN_HASHTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLAN_HASHTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLAN_HASHTABLE` writer"]
pub struct W(crate::W<VLAN_HASHTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLAN_HASHTABLE_SPEC>;
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
impl From<crate::W<VLAN_HASHTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLAN_HASHTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLHT` reader - VLAN Hash Table"]
pub struct VLHT_R(crate::FieldReader<u16, u16>);
impl VLHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VLHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLHT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLHT` writer - VLAN Hash Table"]
pub struct VLHT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&mut self) -> VLHT_W {
        VLHT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds the VLAN Hash Table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlan_hashtable](index.html) module"]
pub struct VLAN_HASHTABLE_SPEC;
impl crate::RegisterSpec for VLAN_HASHTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlan_hashtable::R](R) reader structure"]
impl crate::Readable for VLAN_HASHTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlan_hashtable::W](W) writer structure"]
impl crate::Writable for VLAN_HASHTABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLAN_HASHTABLE to value 0"]
impl crate::Resettable for VLAN_HASHTABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `DATA3_HCMB` reader"]
pub type R = crate::R<Data3HcmbSpec>;
#[doc = "Register `DATA3_HCMB` writer"]
pub type W = crate::W<Data3HcmbSpec>;
#[doc = "Field `BYTE8` reader - Data Byte 8"]
pub type Byte8R = crate::FieldReader;
#[doc = "Field `BYTE8` writer - Data Byte 8"]
pub type Byte8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE7` reader - Data Byte 7"]
pub type Byte7R = crate::FieldReader;
#[doc = "Field `BYTE7` writer - Data Byte 7"]
pub type Byte7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 8"]
    #[inline(always)]
    pub fn byte8(&self) -> Byte8R {
        Byte8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> Byte7R {
        Byte7R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 8"]
    #[inline(always)]
    pub fn byte8(&mut self) -> Byte8W<Data3HcmbSpec> {
        Byte8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> Byte7W<Data3HcmbSpec> {
        Byte7W::new(self, 8)
    }
}
#[doc = "CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_hcmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_hcmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data3HcmbSpec;
impl crate::RegisterSpec for Data3HcmbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3_hcmb::R`](R) reader structure"]
impl crate::Readable for Data3HcmbSpec {}
#[doc = "`write(|w| ..)` method takes [`data3_hcmb::W`](W) writer structure"]
impl crate::Writable for Data3HcmbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA3_HCMB to value 0"]
impl crate::Resettable for Data3HcmbSpec {
    const RESET_VALUE: u32 = 0;
}

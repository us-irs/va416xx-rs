#[doc = "Register `DATA3_CMB6` reader"]
pub type R = crate::R<Data3Cmb6Spec>;
#[doc = "Register `DATA3_CMB6` writer"]
pub type W = crate::W<Data3Cmb6Spec>;
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
    pub fn byte8(&mut self) -> Byte8W<Data3Cmb6Spec> {
        Byte8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> Byte7W<Data3Cmb6Spec> {
        Byte7W::new(self, 8)
    }
}
#[doc = "CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data3Cmb6Spec;
impl crate::RegisterSpec for Data3Cmb6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3_cmb6::R`](R) reader structure"]
impl crate::Readable for Data3Cmb6Spec {}
#[doc = "`write(|w| ..)` method takes [`data3_cmb6::W`](W) writer structure"]
impl crate::Writable for Data3Cmb6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA3_CMB6 to value 0"]
impl crate::Resettable for Data3Cmb6Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DATA0_CMB7` reader"]
pub type R = crate::R<Data0Cmb7Spec>;
#[doc = "Register `DATA0_CMB7` writer"]
pub type W = crate::W<Data0Cmb7Spec>;
#[doc = "Field `BYTE2` reader - Data Byte 2"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE2` writer - Data Byte 2"]
pub type Byte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE1` reader - Data Byte 1"]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE1` writer - Data Byte 1"]
pub type Byte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> Byte2W<Data0Cmb7Spec> {
        Byte2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> Byte1W<Data0Cmb7Spec> {
        Byte1W::new(self, 8)
    }
}
#[doc = "CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0Cmb7Spec;
impl crate::RegisterSpec for Data0Cmb7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0_cmb7::R`](R) reader structure"]
impl crate::Readable for Data0Cmb7Spec {}
#[doc = "`write(|w| ..)` method takes [`data0_cmb7::W`](W) writer structure"]
impl crate::Writable for Data0Cmb7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0_CMB7 to value 0"]
impl crate::Resettable for Data0Cmb7Spec {
    const RESET_VALUE: u32 = 0;
}

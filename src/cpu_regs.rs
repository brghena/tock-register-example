// Example extension of Tock registers. Uses example code from
// andre-richter (https://github.com/andre-richter)

use tock_regs::regs::{IntLike, RegisterLongName, Field, LocalRegisterCopy, FieldValue};

/// Trait for register R/W functions
pub trait RegisterReadWrite<T: IntLike, R: RegisterLongName> {
    #[inline]
    fn get(&self) -> T;

    #[inline]
    fn set(&self, value: T);

    #[inline]
    fn read(&self, field: Field<T, R>) -> T {
        (self.get() & (field.mask << field.shift)) >> field.shift
    }

    #[inline]
    fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }

    #[inline]
    fn modify(&self, field: FieldValue<T, R>) {
        let reg: T = self.get();
        self.set((reg & !field.mask) | field.value);
    }

    #[inline]
    fn modify_no_read(&self, original: LocalRegisterCopy<T, R>, field: FieldValue<T, R>) {
        self.set((original.get() & !field.mask) | field.value);
    }

    #[inline]
    fn is_set(&self, field: Field<T, R>) -> bool {
        self.read(field) != T::zero()
    }

    #[inline]
    fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        self.get() & field.mask != T::zero()
    }

    #[inline]
    fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        self.get() & field.mask == field.value
    }
}

/// Trait for register RO functions
pub trait RegisterRead<T: IntLike, R: RegisterLongName> {
    #[inline]
    fn get(&self) -> T;

    #[inline]
    fn read(&self, field: Field<T, R>) -> T {
        (self.get() & (field.mask << field.shift)) >> field.shift
    }

    #[inline]
    fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    fn is_set(&self, field: Field<T, R>) -> bool {
        self.read(field) != T::zero()
    }

    #[inline]
    fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        self.get() & field.mask != T::zero()
    }

    #[inline]
    fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        self.get() & field.mask == field.value
    }
}

/// Trait for register WO functions
pub trait RegisterWrite<T: IntLike, R: RegisterLongName> {
    #[inline]
    fn set(&self, value: T);

    #[inline]
    fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }
}

pub mod cpu {
    use super::{RegisterReadWrite, RegisterRead, RegisterWrite, IntLike, RegisterLongName};
    use std::marker::PhantomData;

    /// Read/Write registers.
    pub struct ReadWrite<T: IntLike, R: RegisterLongName = ()> {
        __get: fn() -> T,
        __set: unsafe fn(T),
        associated_register: PhantomData<R>,
    }

    /// Read-only registers.
    pub struct ReadOnly<T: IntLike, R: RegisterLongName = ()> {
        __get: fn() -> T,
        associated_register: PhantomData<R>,
    }

    /// Write-only registers.
    pub struct WriteOnly<T: IntLike, R: RegisterLongName = ()> {
        __set: unsafe fn(T),
        associated_register: PhantomData<R>,
    }

    #[allow(dead_code)]
    impl<T: IntLike, R: RegisterLongName> ReadWrite<T, R> {
        pub const fn new(getter: fn() -> T, setter: unsafe fn(T)) -> Self {
            ReadWrite {
                __get: getter,
                __set: setter,
                associated_register: PhantomData,
            }
        }
    }

    impl<T: IntLike, R: RegisterLongName> RegisterReadWrite<T, R> for ReadWrite<T, R> {
        #[inline]
        fn get(&self) -> T {
            (self.__get)()
        }

        #[inline]
        fn set(&self, value: T) {
            unsafe { (self.__set)(value) }
        }
    }

    #[allow(dead_code)]
    impl<T: IntLike, R: RegisterLongName> ReadOnly<T, R> {
        pub const fn new(getter: fn() -> T) -> Self {
            ReadOnly {
                __get: getter,
                associated_register: PhantomData,
            }
        }
    }

    impl<T: IntLike, R: RegisterLongName> RegisterRead<T, R> for ReadOnly<T, R> {
        #[inline]
        fn get(&self) -> T {
            (self.__get)()
        }
    }

    #[allow(dead_code)]
    impl<T: IntLike, R: RegisterLongName> WriteOnly<T, R> {
        pub const fn new(setter: unsafe fn(T)) -> Self {
            WriteOnly {
                __set: setter,
                associated_register: PhantomData,
            }
        }
    }

    impl<T: IntLike, R: RegisterLongName> RegisterWrite<T, R> for WriteOnly<T, R> {
        #[inline]
        fn set(&self, value: T) {
            unsafe { (self.__set)(value) }
        }
    }
}


// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
//方法一 动态分发&dyn
// fn compare_license_types(software:&dyn Licensed, software_two:&dyn Licensed) -> bool {  
    /*对实现同一特性的类型进行比较，需要使用动态分发&dyn,动态分发可以调用实现了该特性的类型的方法，
//     动态分发实现的原理是，编译器在编译时并不知道具体调用的是哪个类型的方法，而是在运行时根据传入的参数动态决定调用哪个类型的方法。
//     在这个例子里，software和software_two是两个实现了Licensed特性的类型，编译器在编译时并不知道具体调用的是哪个类型的方法，而是在运行时根据传入的参数动态决定调用哪个类型的方法。
//     运行时，会根据传入的参数&dyn Licensed来决定调用licensing_info方法的具体实现。
//     如果传入的是SomeSoftware，则调用SomeSoftware的licensing_info方法；如果传入的是OtherSoftware，则调用OtherSoftware的licensing_info方法。
    
//     */
//     software.licensing_info() == software_two.licensing_info()
// }

// 方法二 静态分发
fn compare_license_types<U,T>(software:U, software_two:T) -> bool  
where   
    U:Licensed,
    T:Licensed,
    {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
        /* 为什么用动态分发&dyn,需要加&software和&software_two,因为rust的静态分发是编译时决定的，编译时并不知道具体调用的是哪个类型的方法，而是在运行时根据传入的参数动态决定调用哪个类型的方法。
        采用where U,T:Licensed，是静态分发，需要再编译的时候制定类型的方法。
         */
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}

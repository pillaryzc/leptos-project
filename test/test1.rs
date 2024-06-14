#[cfg(test)]
mod test{
    use std::marker::PhantomData;

    // 基类 Animal
    struct Animal {
        name: String,
    }
    
    // 子类 Cat
    struct Cat {
        name: String,
        is_domestic: bool,
    }
    
    // 泛型结构体 Wrapper，希望 T 是协变的
    struct Wrapper<T> {
        _marker: PhantomData<&T>,
    }
    
    // 动物处理函数
    fn process_animal(animal: &Animal) {
        println!("Processing {}", animal.name);
    }
    
    // 泛型结构体 FunctionWrapper，希望 T 是逆变的
    struct FunctionWrapper<T> {
        func: fn(&T),
        _marker: PhantomData<fn(&T)>,
    }
    
    #[test]
    fn main() {
        let cat = Cat {
            name: "Whiskers".to_string(),
            is_domestic: true,
        };
    
        let wrapped_cat = Wrapper::<Cat> {
            _marker: PhantomData,
        };
    
        // 因为 Wrapper<T> 是协变的，我们可以将 Wrapper<Cat> 安全地转换为 Wrapper<Animal>
        let wrapped_animal: Wrapper<Animal> = wrapped_cat;
    
        let func_wrapper = FunctionWrapper::<Animal> {
            func: process_animal,
            _marker: PhantomData,
        };
    
        // 因为 FunctionWrapper<T> 是逆变的，我们可以将 FunctionWrapper<Animal> 安全地转换为 FunctionWrapper<Cat>
        let cat_func_wrapper: FunctionWrapper<Cat> = func_wrapper;
    }
    
}


// fn find_the_largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for items in list {
//         if items > largest {
//             largest = items
//         }
//     }
//     largest
// }

// 其中 std::cmp::PartialOrd 即为一个 trait。不是所有的 T 本身都适用于 > 进行比较
// 所以使用 std::cmp::PartialOrd 指定拥有该特征的类型。这及时 trait bound 的使用
fn find_the_largest_with_t<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for items in list {
        if items > largest {
            largest = items
        }
    }
    largest
}


fn main() {
    let the_list_1 = vec![1, 3, 4, 100, 120];
    let the_largest_1 = find_the_largest_with_t(&the_list_1); 
    println!("list1 中最大的值为， {}", the_largest_1); // list1 中最大的值为， 120

    let the_list_2 = vec!['a', 'b', 'c', 'd'];
    // shadow 允许再定义一个 the_largest_1 覆盖以前的 the_largest_1
    let the_largest_1 = find_the_largest_with_t(&the_list_2);
    println!("list2 中最大的值为， {}", the_largest_1); // list2 中最大的值为， d

    // let the_list_3: Vec<[&str; 1]> = vec![["1"], ["2"], ["3"]];
    // 报错 [&str] 上没有实现 std::fmt::Display 特征。
    // 并且 only traits defined in the current crate can be implemented for types defined outside of the crate
    // println!("list3 中最大的值为， {}", find_the_largest_with_t(&the_list_3)); 
    
    // 结构体中的泛型
    struct ThePoint<T, U> {
        x: T,
        y: U
    }

    let the_point_1 = ThePoint {
        x: 12,
        y: 0.3
    };

    // 方法中的泛型
    impl<T, U> ThePoint<T, U> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &U {
            &self.y
        }

        // 返回的 ThePoint 中 x 使用当前结构体 x 类型, y 则使用传入的结构体中 y 的类型
        fn mix_up<X, Y> (self, other_point: ThePoint<X, Y>) -> ThePoint<T, Y> {
            ThePoint { x: self.x, y: other_point.y }
        }
    }

    // 特征
    trait GetPoint {
        fn println(&self);
    }

    trait GetSquare {
        fn get_square(&self) -> i32;
    }

    // 所有 ThePoint 结构体创建出来的结构都将拥有 GetPoint 这个特征
    // 特征中的泛型
    impl<T: std::fmt::Display, U: std::fmt::Display> GetPoint for ThePoint<T, U> {
        fn println(&self) {
            println!("当前坐标位置为 ({}, {})", self.x(), self.y())
        }
    }

    // 使用 特征。传入实现了该特征的结构体
    fn use_the_get_point_trait(point: &impl GetPoint) {
        point.println();
    }

    use_the_get_point_trait(&the_point_1);


    impl GetSquare for ThePoint<i32, i32> {
        fn get_square(&self) -> i32 {
            &self.x * &self.y
        }
    }
    let the_i32_point = ThePoint {
        x: 12,
        y: 13,
    };

    // println!("面积为{}", the_i32_point.get_square());

    // 使用 trait bound
    // 定义同时实现了 GetPoint 和 GetSquare 两种特征的 泛型
    fn the_get_square_func<T: GetPoint + GetSquare>(item: &T) {
        item.println();
        println!("面积为{}", item.get_square());
    }
    // 第二种方式 效果一致
    fn the_get_square_func_2(item: &(impl GetPoint + GetSquare)) {
        item.println();
        println!("面积为{}", item.get_square());
    }
    // 第三种方式 通过 where 来指定 trait bound
    fn the_get_square_func_3<T>(item: &T) where T: GetPoint + GetSquare {
        item.println();
        println!("面积为{}", item.get_square());
    }
    the_get_square_func::<ThePoint<i32, i32>>(&the_i32_point);
    the_get_square_func_2(&the_i32_point);
    // the_get_square_func_2(&the_point_1); // 报错 the_point_1 没有实现 GetSquare trait
    the_get_square_func_3(&the_i32_point);

    // 实现了某个 trait 的结构作为 返回值
    fn _return_impl_trait_struct() -> impl GetSquare {
        ThePoint {
            x: 12,
            y: 13,
        }
    }





    // 枚举中的泛型
    // enum Result<T, E> {
    //     OK(T),
    //     Err(E)
    // }

}

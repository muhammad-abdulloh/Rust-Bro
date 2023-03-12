fn main() {
    
    // mod
    // println!("{}", 5 % 2);

    // butun qismini oladikan

    // dastlab bo'lish ko'paytirish qoldiqni olish ishlarkan
    // huddi shu ketma ketlikda.
    // barchasini o'rnini almashtirgan bilan
    // dastlab bo'lish
    // ko'paytirish
    // qoldiq olish ohiri ishlarkan hammasini sinab ko'rildi
    // println!("{}", 5 * 5 % 5 / 3);

    // println!("{}", 0.1 + 0.2); //0.30000000000000004
    // println!("{}", 115.19999999999999 + 0.2); //115.39999999999999
    // println!("{}", 115.19999999999999 + 0.1); //115.29999999999998
    // println!("{}", 115.19999999999999 + 0.3); //115.39999999999999
    // println!("{}", 115.19999999999999 + 0.4); //115.6
    // println!("{}", 115.19999999999999 + 0.5);
    // println!("{}", 115.19999999999999 + 0.6);
    // println!("{}", 115.19999999999999 + 0.7);
    // println!("{}", 115.19999999999999 + 0.8);
    // println!("{}", 115.19999999999999 + 0.9);
    // println!("{}", 115.19999999999999 + 1f32);
    // println!("{}", 115.19999999999999 + 1.0);

    // println!("{}", 2.1f32 + 2.);
    // 7. == 7.0 => ga mashnaqa huddi CSS ga o'xsharkan
    // println!("{}", 600./7.);   // 85.71428571428571
    // println!("{}", 600.0/7.0); // 85.71428571428571
    // println!("{}", 600/7);     // 85
    // println!("{}", -600/7);  // -85
    // println!("{}", -600/-7); // 85

    // println!("{}", 600%7); // 5
    // println!("{}", -600%7); // -5
    // println!("{}", -600%-7); // -5

    // println!("{}", "The 
    //     Second
    //     Line");
    //     /*
    //     The 
    //             Second
    //             Line */

    // // \ teskari slashdan kegin enter tashlansa anashu qatorni bir qatorda qilib qo'yadi ekan
    // // agarda tashlanmasa error beradikan
    // println!("{}", "This \
    //     just \
    //     one \
    //     line"); // This is just one line

    // let mut number = 23;
    // println!("{}", number); //23

    // number = 43;

    // println!("{}", number); //43

    // let number = 22;
    // println!("{}", number); //22

    // number = 54;
    // println!("{}", number); /// error developers but qayta elon qilindi o'zgaruvchi

    // mut kalit so'zi bilan variable elon qilsakda
    // anashu variableni qayta boshqa qiymat o'zlashtirib va anshu yangi 
    // o'zlashtirilgan variableni ishlatmaguncha error bersi
    // let mut number = 12; 
    // println!("{}", number); // warning chiqadi
    // // number = 21;  // shu ish qilinsa
    // // println!("{}", number);  // va uni print qilinsa warning yo'qoladi :) velcom to Rust 
   
    // #[warn(unused_mut)]
    // let mut number = 23;  // warningni yo'qotmoqchidm ishlamadi :)
    // println!("{}", number);

    // let number1;
    // //println!("{}", number1); // tekkiz error chiqaturg'o'y
    
    // number1 = 121; // pastda tenglashtirib qo'ysagam error chiqadi
    // println!("{}", number1); // shu printni pastga qo'ysak error yo'q bo'ladi

    // let _ss = 12;
    // println!("{}", _ss); // 22

    println!("{} {} {} {} {} ", 4 == 4, 4 != 4, 4 < 4, 4 <= 4, 4 > 4 );

    println!("{} ", "I love Rust :)");

}

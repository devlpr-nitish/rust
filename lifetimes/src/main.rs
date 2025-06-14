

fn main(){
    let str1 = String::from("Nitish");
    let ans;
    {
        let str2 = String::from("Kumar");
        // here ans can be either str1 or str2 but scope of str2 is in this bracket only 
        // and if str2 will be answer it means fn return str2 , ans point to str2 and we are trying to access str2 
        // out of this block so it is not possible, compiler will complain
        ans = longest_string(&str1, &str2);
    }

    // println!("{}", ans);
}
// giving both stirngs to the one life times but it will take only smaller life time
fn longest_string<'a>(s1 : &'a String, s2: &'a String) -> &'a String{
    if s1.len() > s2.len(){
        return s1
    }

    return s2
}
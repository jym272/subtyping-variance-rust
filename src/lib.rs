mod phantom;

//mutable reference to a string
//two lifetimes: 'a -> for the mutable borrow
// and 'b -> the string that is being borrowed,
//  and return the lifetime of the string   that is being borrowed
pub fn strtok<'b>(s: &'_ mut &'b str, delimiter: char) -> &'b str {
    if let Some(index_of_delimiter) = s.find(delimiter) {
        let token = &s[..index_of_delimiter];
        *s = &s[index_of_delimiter + delimiter.len_utf8()..];
        token
    } else {
        let token = *s;
        *s = "";
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hello_world() {
        let mut s = "hello world";
        let z = &mut s; //mut ref are covariant in their lifetime, the borrow is shorter
                        //from static to 'z
                        //stop the borrow of z and start a new one
        let hello = strtok(&mut s, ' ');

        assert_eq!(hello, "hello");
        //as long as hello lives, s continues to be mut borrowed
        assert_eq!(s, "world"); //s is still mutable
    }
    #[test]
    fn foo() {
        //mut ref are invariant in T, and covariant in 'a (lifetime)
        let mut y = true;
        let mut z = &mut y;
        let x = Box::new(true);
        let x: &'static mut bool = Box::leak(x);
        z = x;
        //is ok to shorted the lifetime of x
    }
}

//T: U
//T is at least as useful as U //T is a subtype of U

// Notation: <: -> 'is a subtype of'

//covariance: allow to shorten the borrow
// 'static <: 'a
// &'static T <: &'a T

// &'static str //most useful lifetime
// &'a str

//contravariance only one example: functions
// 'static <: 'a
// Fn(&'a T) <: Fn(&'static T)
// Fn(&'a str) //more useful -> can take a shorter lifetime
// Fn(&'static str) ->more strictly, least useful

//invariance
// you must provide a lifetime that is the exact same lifetime as the lifetime of the reference

// &'a mut &'b T
//mut ref are invariant in T, and covariant in 'a (lifetime)

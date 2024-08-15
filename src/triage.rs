/// ## Tri! - Try Into ##
///
/// The **tri!** macro is a tool for handling results and options.
/// Unlike the `?` operator, **tri!** allows you to easily specify
/// what to do if unpacking fails.
///
///     tri!(a => b $$ c);
///
///     a - Leading Expression
///     b - Specified Term
///     c - Trailing Expression
///
///     $$ - Tri Operator
///
/// ### Tri-Fail `->`
///
/// Automatically returns the trailing expression in an error if
/// the leading expression doesn't match the specified term.
///
///     // Tri Expression
///     tri!(item => Some(value) -> "Item was None!");
///
///     // Expanded Form
///     if let Some(value) = item { value }
///     else { return Err("Item was None!"); }
///
/// ### Tri-Fall `<>`
///
/// Evaluates and uses the trailing expression as a fallback if
/// the leading expression doesn't match the specified term.
///
///     // Tri Expression
///     tri!(item => Some(value) <> backup);
///
///     // Expanded Form
///     if let Some(value) = item { value }
///     else { backup }
///
/// ### Tri-Return `#>`
///
/// Similar to the `->` operator, but it doesn't wrap the return
/// in an error.
///
///     // Tri Expression
///     tri!(item => Some(value) #> core::result::Result::Err(()));
///
///     // Expanded Form
///     if let Some(value) = item { value }
///     else { return core::result::Result::Err(()); }
///
/// ### Tri-Return `#> break`
///
/// Adding a **break** expression immediately after the operator
/// results in a break being called rather than a return. A lifetime
/// can also be specified, and a trailing expression will be specified
/// as well.
///
///     // Tri Expression
///     tri!(item => Some(value) #> break 'a true);
///
///     // Expanded Form
///     if let Some(value) = item { value }
///     else { break 'a true; }
///
/// ### Tri-Until `%>`
///
/// Performs the leading expression until its output matches the
/// specified term.
///
///     // Tri Expression
///     tri!(item => Some(value) %> thing += 1);
///
///     // Expanded Form
///     loop {
///         if let Some(value) = item { break value; }
///         else { thing += 1; }
///     }
///
/// ### Tri-While `>>`
///
/// This operator acts like a `do-while` loop. The values are
/// initialized with the given expressions, and the trailing
/// expression is evaluated. The leading expression is then
/// evaluated in a loop. For every time that the output matches
/// the given variant, the trailing expression is evaluated with
/// those values.
///
///     // Tri Expression
///     tri!(do_stuff(number) => Some(value = 0) >> number += value);
///
///     // Expanded Form (Pseudo-Code)
///     do(value = 0) { number += value; }
///     while let Some(value) = do_stuff(number);
///
/// ___
#[macro_export]
macro_rules! tri {
    // Caption
    ($chk:expr => $($xpv:ident $(::<$($inr:tt)+>)?)::+[$($uci:tt)+ $(,)?] $($tal:tt)+) =>
    { $crate::__format_caption!($chk => $($xpv $(::<$($inr)+>)?)::+ [$($uci)+] [] [] $($tal)+); };
    
    // Variant
    ($chk:expr => $($xpv:ident $(::<$($inr:tt)+>)?)::+($($uci:tt)+ $(,)?) $($tal:tt)+) =>
    { $crate::__format_variant!($chk => $($xpv $(::<$($inr)+>)?)::+ [$($uci)+] [] [] $($tal)+) };
    
    // Path
    ($chk:expr => $($xpv:ident $(::<$($inr:tt)+>)?)::+ $($tal:tt)+) =>
    { $crate::__expand_path!($chk => $($xpv $(::<$($inr)+>)?)::+ [] $($tal)+); };
    
    // Rule
    ($chk:expr => [$($rle:pat),*] $($tal:tt)+) =>
    { $crate::__expand_rule!($chk => [$($rle),*] $($tal)+); };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __format_caption {
    // Ref Mut
    (
        $chk:expr => $xpv:path
        [ref mut $a:ident $(: $(&)? $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_caption! {
            $chk => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                ref, mut, $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* ref mut # $a $(= $c)?]
            $($tal)+
        }
    };
    
    // Ref
    (
        $chk:expr => $xpv:path
        [ref $a:ident $(: $(&)? $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_caption! {
            $chk => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                ref, , $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* ref # $a $(= $c)?]
            $($tal)+
        }
    };
    
    // Note - Removed Mut From First Sequence On Its Own
    // Mut
    (
        $chk:expr => $xpv:path
        [mut $a:ident $(: $(&)? $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_caption! {
            $chk => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                , , $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* mut # $a $(= $c)?]
            $($tal)+
        }
    };
    
    // $ident
    (
        $chk:expr => $xpv:path
        [$a:ident $(: $(&)? $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_caption! {
            $chk => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                , , $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* # $a $(= $c)?]
            $($tal)+
        }
    };
    
    // $pat
    (
        $chk:expr => $xpv:path
        [$wut:pat $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_caption! {
            $chk => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                , , , $wut
            ]
            [$($($bmo)* # $cln $(= $ani)?),*]
            $($tal)+
        }
    };
    
    // Output
    (
        $chk:expr => $xpv:path
        []
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__expand_caption! {
            $chk => $xpv
            [$($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?),*]
            [$($($bmo)* # $cln $(= $ani)?),*]
            $($tal)+
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __format_variant {
    // Ref Mut
    (
        $chc:expr => $xpv:path
        [ref mut $a:ident $(: $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_variant! {
            $chc => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                ref, mut, $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* ref mut # $a]
            $($tal)+
        }
    };
    
    // Ref
    (
        $chc:expr => $xpv:path
        [ref $a:ident $(: $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_variant! {
            $chc => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                ref, , $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* ref # $a]
            $($tal)+
        }
    };
    
    // Note - Removed Mut From First Sequence
    // Mut
    (
        $chc:expr => $xpv:path
        [mut $a:ident $(: $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_variant! {
            $chc => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                , , $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* mut # $a]
            $($tal)+
        }
    };
    
    // $ident
    (
        $chc:expr => $xpv:path
        [$a:ident $(: $($_ty:ident $(::<$($owo:tt)+>)?)::+)? $(@ $b:pat)? $(= $c:expr)? $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_variant! {
            $chc => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                , , $a $(@ $b)? $(= $c)?,
            ]
            [$($($bmo)* # $cln $(= $ani)?,)* # $a]
            $($tal)+
        }
    };
    
    // $pat
    (
        $chc:expr => $xpv:path
        [$wut:pat $(, $($uci:tt)+)?]
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__format_variant! {
            $chc => $xpv
            [$($($uci)+)?]
            [
                $($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?,)*
                , , , $wut
            ]
            [$($($bmo)* # $cln $(= $ani)?),*]
            $($tal)+
        }
    };
    
    // Output
    (
        $chc:expr => $xpv:path
        []
        [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? $(= $ini:expr)?)?, $($alt:pat)?),*]
        [$($($bmo:ident)* # $cln:ident $(= $ani:expr)?),*]
        $($tal:tt)+
    ) => {
        $crate::__expand_variant! {
            $chc => $xpv
            [$($($rfi)?, $($mti)?, $($var $(@ $grd)? $(= $ini)?)?, $($alt)?),*]
            [$($($bmo)* # $cln $(= $ani)?),*]
            $($tal)+
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_caption {
    // Tri-While
    ($chk:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? = $ini:expr)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident = $ani:expr),*] >> $inc:expr $(;)?) =>
    {
        let $($($bmo)* $cln),* = {
            let mut __loop_monitor_dont_use_this_variable_please = ($($ani),*);
            loop {
                let $($($bmo)* $cln),+ = __loop_monitor_dont_use_this_variable_please;
                $inc;
                let $xpv($($($rfi)? $($mti)? $($var $(@ $grd)?)? $($alt)?),*) = $chk else { break ($($cln),*) };
                __loop_monitor_dont_use_this_variable_please = ($($cln),*);
            }
        };
    };
    
    // Tri-Fail
    ($chk:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] -> $otw:expr $(;)?) =>
    { let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chk else { return ::std::result::Result::Err($otw) }; };
    
    // Tri-Fall
    ($chk:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] <> $($otw:expr $(;)?),+) =>
    { let $($($bmo)* $cln),* = if let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chk { ($($cln),*) } else { ($($otw),+) }; };
    
    // Tri-Return (Break)
    ($chk:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] #> break $($tal:tt)*) =>
    { let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chk else { break $($tal)* }; };
    
    // Tri-Return
    ($chk:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] #> $otw:expr $(;)?) =>
    { let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chk else { return $otw }; };
    
    // Tri-Until
    ($chk:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] %> $otw:expr $(;)?) =>
    { let($($($bmo)* $cln),*) = loop { if let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chk { break ($($cln),*) } else { $otw; } }; };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_variant {
    // Tri-While
    ($chc:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)? = $ini:expr)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident = $ani:expr),*] >> $inc:expr $(;)?) =>
    {
        {
            let mut __loop_monitor_dont_use_this_variable_please = ($($ani),*);
            loop {
                let ($($($bmo)* $cln),*) = __loop_monitor_dont_use_this_variable_please;
                $inc;
                let $xpv($($($rfi)? $($mti)? $($var $(@ $grd)?)? $($alt)?),*) = $chc else { break };
                __loop_monitor_dont_use_this_variable_please = ($($cln),*);
            }
        }
    };
    
    // Tri-Fail
    ($chc:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] -> $otw:expr $(;)?) =>
    { if let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chc { ($($cln),*) } else { return ::std::result::Result::Err($otw) } };
    
    // Tri-Fall
    ($chc:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] <> $($otw:expr $(;)?),+ $(,)?) =>
    { if let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chc { ($($cln),*) } else { ($($otw),+) } };
    
    // Tri-Return (Break)
    ($chc:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] #> break $($tal:tt)*) =>
    { if let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chc { ($($cln),*) } else { break $($tal)* } };
    
    // Tri-Return
    ($chc:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] #> $otw:expr $(;)?) =>
    { if let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chc { ($($cln),*) } else { return $otw }; };
    
    // Tri-Until
    ($chc:expr => $xpv:path [$($($rfi:ident)?, $($mti:ident)?, $($var:ident $(@ $grd:pat)?)?, $($alt:pat)?),*] [$($($bmo:ident)* # $cln:ident),*] %> $otw:expr $(;)?) =>
    { loop { if let $xpv($($($rfi)* $($mti)? $($var $(@ $grd)?)? $($alt)?),+) = $chc { break ($($cln),*) } else { $otw; } } };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_path {
    // Tri-While
    ($chc:expr => $xpv:path [] >> $inc:expr $(;)?) =>
    { loop { $inc; let $xpv = $chc else { break }; } };
    
    // Tri-Fail
    ($chc:expr => $xpv:path [] -> $otw:expr $(;)?) =>
    { let $xpv = $chc else { return ::std::result::Result::Err($otw) }; };
    
    // Tri-Fall
    ($chc:expr => $xpv:path [] <> $otw:expr $(;)?) =>
    { match $chc { $xpv => (), _ => { $otw; } } };
    
    // Tri-Return (Break)
    ($chc:expr => $xpv:path [] #> break $($tal:tt)*) =>
    { let $xpv = $chc else { break $($tal)* }; };
    
    // Tri-Return
    ($chc:expr => $xpv:path [] #> $otw:expr $(;)?) =>
    { let $xpv = $chc else { return $otw }; };
    
    // Tri-Until
    ($chc:expr => $xpv:path [] %> $otw:expr $(;)?) =>
    { loop { if let $xpv = $chc { break } else { $otw; } } };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_rule {
    // Tri-While
    ($chc:expr => [$($rle:pat),+] >> $inc:expr $(;)?) =>
    { loop { $inc; let ($($rle),+) = $chc else { break }; } };
    
    // Tri-Fail
    ($chc:expr => [$($rle:pat),+] -> $otw:expr $(;)?) =>
    { let ($($rle),+) = $chc else { return ::std::result::Result::Err($otw) }; };
    
    // Tri-Fall
    ($chc:expr => [$($rle:pat),+] <> $otw:expr $(;)?) =>
    { match $chc { ($($rle),+) => (), _ => { $otw } } };
    
    // Tri-Return (Break)
    ($chc:expr => [$($rle:pat),+] #> break $($tal:tt)*) =>
    { let ($($rle),+) = $chc else { break $($tal)* }; };
    
    // Tri-Return
    ($chc:expr => [$($rle:pat),+] #> $otw:expr $(;)?) =>
    { let ($($rle),+) = $chc else { return $otw }; };
    
    // Tri-Until
    ($chc:expr => [$($rle:pat),+] %> $otw:expr $(;)?) =>
    { loop { if let $($rle),+ = $chc { break } else { $otw } } };
}

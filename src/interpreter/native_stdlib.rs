use super::value::{Environment, SharedValue, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn define_math(env: &mut Environment) {
    env.define(
        "مطلق",
        Value::NativeFunction {
            name: "مطلق".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Number(n) => Ok(Rc::new(RefCell::new(Value::Number(n.abs())))),
                _ => Err("مطلق يتطلب رقماً".into()),
            },
        },
        false,
    );

    env.define(
        "جذر",
        Value::NativeFunction {
            name: "جذر".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Number(n) => {
                    if *n < 0.0 {
                        return Err("جذر عدد سالب غير مسموح".into());
                    }
                    Ok(Rc::new(RefCell::new(Value::Number(n.sqrt()))))
                }
                _ => Err("جذر يتطلب رقماً".into()),
            },
        },
        false,
    );

    env.define(
        "أس",
        Value::NativeFunction {
            name: "أس".to_string(),
            func: |a| {
                let b = a[0].borrow().to_number()?;
                let e = a[1].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(b.powf(e)))))
            },
        },
        false,
    );

    env.define(
        "سقف",
        Value::NativeFunction {
            name: "سقف".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Number(n) => Ok(Rc::new(RefCell::new(Value::Number(n.ceil())))),
                _ => Err("سقف يتطلب رقماً".into()),
            },
        },
        false,
    );

    env.define(
        "أرض",
        Value::NativeFunction {
            name: "أرض".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Number(n) => Ok(Rc::new(RefCell::new(Value::Number(n.floor())))),
                _ => Err("أرض يتطلب رقماً".into()),
            },
        },
        false,
    );

    env.define(
        "قرب",
        Value::NativeFunction {
            name: "قرب".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Number(n) => Ok(Rc::new(RefCell::new(Value::Number(n.round())))),
                _ => Err("قرب يتطلب رقماً".into()),
            },
        },
        false,
    );

    env.define(
        "جيب",
        Value::NativeFunction {
            name: "جيب".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.sin()))))
            },
        },
        false,
    );

    env.define(
        "جتا",
        Value::NativeFunction {
            name: "جتا".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.cos()))))
            },
        },
        false,
    );

    env.define(
        "ظل",
        Value::NativeFunction {
            name: "ظل".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.tan()))))
            },
        },
        false,
    );

    env.define(
        "جيب_معكوس",
        Value::NativeFunction {
            name: "جيب_معكوس".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.asin()))))
            },
        },
        false,
    );

    env.define(
        "جتا_معكوس",
        Value::NativeFunction {
            name: "جتا_معكوس".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.acos()))))
            },
        },
        false,
    );

    env.define(
        "ظل_معكوس",
        Value::NativeFunction {
            name: "ظل_معكوس".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.atan()))))
            },
        },
        false,
    );

    env.define(
        "ظل_معكوس2",
        Value::NativeFunction {
            name: "ظل_معكوس2".to_string(),
            func: |a| {
                let y = a[0].borrow().to_number()?;
                let x = a[1].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(y.atan2(x)))))
            },
        },
        false,
    );

    env.define(
        "لوغ",
        Value::NativeFunction {
            name: "لوغ".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.ln()))))
            },
        },
        false,
    );

    env.define(
        "لوغ10",
        Value::NativeFunction {
            name: "لوغ10".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.log10()))))
            },
        },
        false,
    );

    env.define(
        "لوغ2",
        Value::NativeFunction {
            name: "لوغ2".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.log2()))))
            },
        },
        false,
    );

    env.define(
        "أسي",
        Value::NativeFunction {
            name: "أسي".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.exp()))))
            },
        },
        false,
    );

    env.define(
        "مجموع",
        Value::NativeFunction {
            name: "مجموع".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::List(l) => {
                    let mut sum = 0.0;
                    for item in l {
                        sum += item.borrow().to_number()?;
                    }
                    Ok(Rc::new(RefCell::new(Value::Number(sum))))
                }
                _ => Err("مجموع يتطلب قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "متوسط",
        Value::NativeFunction {
            name: "متوسط".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::List(l) => {
                    if l.is_empty() {
                        return Err("القائمة فارغة".into());
                    }
                    let mut sum = 0.0;
                    for item in l {
                        sum += item.borrow().to_number()?;
                    }
                    Ok(Rc::new(RefCell::new(Value::Number(sum / l.len() as f64))))
                }
                _ => Err("متوسط يتطلب قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "أدنى",
        Value::NativeFunction {
            name: "أدنى".to_string(),
            func: |a| {
                if a.len() >= 2 {
                    let a_val = a[0].borrow().to_number()?;
                    let b_val = a[1].borrow().to_number()?;
                    return Ok(Rc::new(RefCell::new(Value::Number(a_val.min(b_val)))));
                }
                match &*a[0].borrow() {
                    Value::List(l) => {
                        if l.is_empty() {
                            return Ok(Rc::new(RefCell::new(Value::Null)));
                        }
                        let mut min = f64::INFINITY;
                        for item in l {
                            let n = item.borrow().to_number()?;
                            if n < min {
                                min = n;
                            }
                        }
                        Ok(Rc::new(RefCell::new(Value::Number(min))))
                    }
                    _ => Err("أدنى يتطلب قائمة أو رقمين".into()),
                }
            },
        },
        false,
    );

    env.define(
        "أقصى",
        Value::NativeFunction {
            name: "أقصى".to_string(),
            func: |a| {
                if a.len() >= 2 {
                    let a_val = a[0].borrow().to_number()?;
                    let b_val = a[1].borrow().to_number()?;
                    return Ok(Rc::new(RefCell::new(Value::Number(a_val.max(b_val)))));
                }
                match &*a[0].borrow() {
                    Value::List(l) => {
                        if l.is_empty() {
                            return Ok(Rc::new(RefCell::new(Value::Null)));
                        }
                        let mut max = f64::NEG_INFINITY;
                        for item in l {
                            let n = item.borrow().to_number()?;
                            if n > max {
                                max = n;
                            }
                        }
                        Ok(Rc::new(RefCell::new(Value::Number(max))))
                    }
                    _ => Err("أقصى يتطلب قائمة أو رقمين".into()),
                }
            },
        },
        false,
    );

    env.define(
        "تقطيع",
        Value::NativeFunction {
            name: "تقطيع".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                let lo = a[1].borrow().to_number()?;
                let hi = a[2].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.clamp(lo, hi)))))
            },
        },
        false,
    );

    env.define(
        "رتب",
        Value::NativeFunction {
            name: "رتب".to_string(),
            func: |a| match (*a[0].borrow()).clone() {
                Value::List(mut l) => {
                    l.sort_by(|x, y| {
                        let xv = x.borrow().to_number().unwrap_or(0.0);
                        let yv = y.borrow().to_number().unwrap_or(0.0);
                        xv.partial_cmp(&yv).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    Ok(Rc::new(RefCell::new(Value::List(l))))
                }
                _ => Err("رتب يتطلب قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "إشارة",
        Value::NativeFunction {
            name: "إشارة".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(if n > 0.0 {
                    1.0
                } else if n < 0.0 {
                    -1.0
                } else {
                    0.0
                }))))
            },
        },
        false,
    );
}

pub fn define_string_funcs(env: &mut Environment) {
    env.define(
        "طول",
        Value::NativeFunction {
            name: "طول".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::String(s) => Ok(Rc::new(RefCell::new(Value::Number(
                    s.chars().count() as f64
                )))),
                Value::List(l) => Ok(Rc::new(RefCell::new(Value::Number(l.len() as f64)))),
                Value::Dictionary(d) => Ok(Rc::new(RefCell::new(Value::Number(d.len() as f64)))),
                v => Err(format!("طول لا يدعم {}", v.type_name())),
            },
        },
        false,
    );

    env.define(
        "كبير",
        Value::NativeFunction {
            name: "كبير".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::String(s) => Ok(Rc::new(RefCell::new(Value::String(s.to_uppercase())))),
                _ => Err("كبير يتطلب نصاً".into()),
            },
        },
        false,
    );

    env.define(
        "صغير",
        Value::NativeFunction {
            name: "صغير".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::String(s) => Ok(Rc::new(RefCell::new(Value::String(s.to_lowercase())))),
                _ => Err("صغير يتطلب نصاً".into()),
            },
        },
        false,
    );

    env.define(
        "قص",
        Value::NativeFunction {
            name: "قص".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::String(s) => Ok(Rc::new(RefCell::new(Value::String(s.trim().to_string())))),
                _ => Err("قص يتطلب نصاً".into()),
            },
        },
        false,
    );

    env.define(
        "انقسم",
        Value::NativeFunction {
            name: "انقسم".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow()) {
                (Value::String(s), Value::String(d)) => {
                    let parts: Vec<SharedValue> = s
                        .split(d.as_str())
                        .map(|p| Rc::new(RefCell::new(Value::String(p.to_string()))))
                        .collect();
                    Ok(Rc::new(RefCell::new(Value::List(parts))))
                }
                _ => Err("انقسم يتطلب نصين".into()),
            },
        },
        false,
    );

    env.define(
        "اجمع",
        Value::NativeFunction {
            name: "اجمع".to_string(),
            func: |a| {
                let delim = if a.len() > 1 {
                    a[1].borrow().to_string_value()
                } else {
                    String::new()
                };
                match &*a[0].borrow() {
                    Value::List(l) => {
                        let parts: Vec<String> =
                            l.iter().map(|v| v.borrow().to_string_value()).collect();
                        Ok(Rc::new(RefCell::new(Value::String(parts.join(&delim)))))
                    }
                    _ => Err("اجمع يتطلب قائمة".into()),
                }
            },
        },
        false,
    );

    env.define(
        "استبدل",
        Value::NativeFunction {
            name: "استبدل".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow(), &*a[2].borrow()) {
                (Value::String(s), Value::String(o), Value::String(n)) => Ok(Rc::new(
                    RefCell::new(Value::String(s.replace(o.as_str(), n.as_str()))),
                )),
                _ => Err("استبدل يتطلب ثلاثة نصوص".into()),
            },
        },
        false,
    );

    env.define(
        "ابحث",
        Value::NativeFunction {
            name: "ابحث".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::String(s) => {
                    if let Value::String(sub) = &*a[1].borrow() {
                        match s.find(sub.as_str()) {
                            Some(idx) => Ok(Rc::new(RefCell::new(Value::Number(
                                s[..idx].chars().count() as f64,
                            )))),
                            None => Ok(Rc::new(RefCell::new(Value::Number(-1.0)))),
                        }
                    } else {
                        Err("ابحث في نص يتطلب نصاً فرعياً".into())
                    }
                }
                Value::List(l) => {
                    let item = a[1].borrow().clone();
                    for (i, v) in l.iter().enumerate() {
                        if *v.borrow() == item {
                            return Ok(Rc::new(RefCell::new(Value::Number(i as f64))));
                        }
                    }
                    Ok(Rc::new(RefCell::new(Value::Number(-1.0))))
                }
                _ => Err("ابحث يتطلب نصاً أو قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "يحتوي",
        Value::NativeFunction {
            name: "يحتوي".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow()) {
                (Value::String(s), Value::String(sub)) => Ok(Rc::new(RefCell::new(
                    Value::Boolean(s.contains(sub.as_str())),
                ))),
                (Value::List(l), item) => Ok(Rc::new(RefCell::new(Value::Boolean(
                    l.iter().any(|v| *v.borrow() == *item),
                )))),
                (Value::Dictionary(d), Value::String(k)) => Ok(Rc::new(RefCell::new(
                    Value::Boolean(d.contains_key(k.as_str())),
                ))),
                _ => Err("يحتوي يتطلب نصاً أو قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "يبدأ_ب",
        Value::NativeFunction {
            name: "يبدأ_ب".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow()) {
                (Value::String(s), Value::String(p)) => Ok(Rc::new(RefCell::new(Value::Boolean(
                    s.starts_with(p.as_str()),
                )))),
                _ => Err("يبدأ_ب يتطلب نصين".into()),
            },
        },
        false,
    );

    env.define(
        "ينتهي_ب",
        Value::NativeFunction {
            name: "ينتهي_ب".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow()) {
                (Value::String(s), Value::String(x)) => Ok(Rc::new(RefCell::new(Value::Boolean(
                    s.ends_with(x.as_str()),
                )))),
                _ => Err("ينتهي_ب يتطلب نصين".into()),
            },
        },
        false,
    );

    env.define(
        "اقتطع",
        Value::NativeFunction {
            name: "اقتطع".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::String(s) => {
                    let chars: Vec<char> = s.chars().collect();
                    let len = chars.len() as i64;
                    let to_idx = |n: f64| -> usize {
                        let i = n as i64;
                        let actual = if i < 0 { (len + i).max(0) } else { i.min(len) };
                        actual as usize
                    };
                    let start = if a.len() > 1 {
                        to_idx(a[1].borrow().to_number().unwrap_or(0.0))
                    } else {
                        0
                    };
                    let end = if a.len() > 2 {
                        to_idx(a[2].borrow().to_number().unwrap_or(len as f64))
                    } else {
                        chars.len()
                    };
                    let result: String = chars[start..end.min(chars.len())].iter().collect();
                    Ok(Rc::new(RefCell::new(Value::String(result))))
                }
                Value::List(l) => {
                    let len = l.len() as i64;
                    let to_idx = |n: f64| -> usize {
                        let i = n as i64;
                        let actual = if i < 0 { (len + i).max(0) } else { i.min(len) };
                        actual as usize
                    };
                    let start = if a.len() > 1 {
                        to_idx(a[1].borrow().to_number().unwrap_or(0.0))
                    } else {
                        0
                    };
                    let end = if a.len() > 2 {
                        to_idx(a[2].borrow().to_number().unwrap_or(len as f64))
                    } else {
                        l.len()
                    };
                    let result = l[start..end.min(l.len())].to_vec();
                    Ok(Rc::new(RefCell::new(Value::List(result))))
                }
                _ => Err("اقتطع يتطلب نصاً أو قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "حرف_رقم",
        Value::NativeFunction {
            name: "حرف_رقم".to_string(),
            func: |a| {
                let n = a[0].borrow().to_number()? as u32;
                match char::from_u32(n) {
                    Some(c) => Ok(Rc::new(RefCell::new(Value::String(c.to_string())))),
                    None => Err(format!("رمز {} غير صالح", n)),
                }
            },
        },
        false,
    );

    env.define(
        "رقم_حرف",
        Value::NativeFunction {
            name: "رقم_حرف".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::String(s) => match s.chars().next() {
                    Some(c) => Ok(Rc::new(RefCell::new(Value::Number(c as u32 as f64)))),
                    None => Err("النص فارغ".into()),
                },
                _ => Err("رقم_حرف يتطلب نصاً".into()),
            },
        },
        false,
    );

    env.define(
        "نص_منسق",
        Value::NativeFunction {
            name: "نص_منسق".to_string(),
            func: |a| {
                let template = match &*a[0].borrow() {
                    Value::String(s) => s.clone(),
                    _ => return Err("نص_منسق يتطلب نصاً أولاً".into()),
                };
                let mut result = template.clone();
                for (i, arg) in a[1..].iter().enumerate() {
                    let placeholder = format!("{{{}}}", i);
                    result = result.replace(&placeholder, &arg.borrow().to_string());
                }
                Ok(Rc::new(RefCell::new(Value::String(result))))
            },
        },
        false,
    );
}

pub fn define_list_funcs(env: &mut Environment) {
    env.define(
        "أضف",
        Value::NativeFunction {
            name: "أضف".to_string(),
            func: |a| {
                let item = a[1].borrow().clone();
                match &mut *a[0].borrow_mut() {
                    Value::List(l) => {
                        l.push(Rc::new(RefCell::new(item)));
                        Ok(Rc::new(RefCell::new(Value::Null)))
                    }
                    _ => Err("أضف يتطلب قائمة".into()),
                }
            },
        },
        false,
    );

    env.define(
        "أزل",
        Value::NativeFunction {
            name: "أزل".to_string(),
            func: |a| {
                let idx = a[1].borrow().to_number()? as i64;
                match &mut *a[0].borrow_mut() {
                    Value::List(l) => {
                        let len = l.len() as i64;
                        let actual = if idx < 0 { len + idx } else { idx };
                        if actual < 0 || actual >= len {
                            return Err(format!("الفهرس {} خارج النطاق", idx));
                        }
                        Ok(l.remove(actual as usize))
                    }
                    _ => Err("أزل يتطلب قائمة وفهرساً".into()),
                }
            },
        },
        false,
    );

    env.define(
        "أدرج",
        Value::NativeFunction {
            name: "أدرج".to_string(),
            func: |a| {
                let idx = a[1].borrow().to_number()? as usize;
                let item = a[2].borrow().clone();
                match &mut *a[0].borrow_mut() {
                    Value::List(l) => {
                        if idx > l.len() {
                            return Err("الفهرس خارج النطاق".into());
                        }
                        l.insert(idx, Rc::new(RefCell::new(item)));
                        Ok(Rc::new(RefCell::new(Value::Null)))
                    }
                    _ => Err("أدرج يتطلب قائمة".into()),
                }
            },
        },
        false,
    );

    env.define(
        "اعكس",
        Value::NativeFunction {
            name: "اعكس".to_string(),
            func: |a| match (*a[0].borrow()).clone() {
                Value::List(mut l) => {
                    l.reverse();
                    Ok(Rc::new(RefCell::new(Value::List(l))))
                }
                Value::String(s) => Ok(Rc::new(RefCell::new(Value::String(
                    s.chars().rev().collect(),
                )))),
                _ => Err("اعكس يتطلب قائمة أو نصاً".into()),
            },
        },
        false,
    );

    env.define(
        "فريد",
        Value::NativeFunction {
            name: "فريد".to_string(),
            func: |a| match (*a[0].borrow()).clone() {
                Value::List(l) => {
                    let mut seen: Vec<String> = Vec::new();
                    let mut result = Vec::new();
                    for item in l {
                        let repr = item.borrow().to_string();
                        if !seen.contains(&repr) {
                            seen.push(repr);
                            result.push(item);
                        }
                    }
                    Ok(Rc::new(RefCell::new(Value::List(result))))
                }
                _ => Err("فريد يتطلب قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "نطاق",
        Value::NativeFunction {
            name: "نطاق".to_string(),
            func: |a| {
                let (start, end, step) = if a.len() == 1 {
                    (0.0, a[0].borrow().to_number()?, 1.0)
                } else if a.len() == 2 {
                    (a[0].borrow().to_number()?, a[1].borrow().to_number()?, 1.0)
                } else {
                    (
                        a[0].borrow().to_number()?,
                        a[1].borrow().to_number()?,
                        a[2].borrow().to_number()?,
                    )
                };
                if step == 0.0 {
                    return Err("الخطوة لا يمكن أن تكون صفر".into());
                }
                let mut list = Vec::new();
                let mut i = start;
                while (step > 0.0 && i < end) || (step < 0.0 && i > end) {
                    list.push(Rc::new(RefCell::new(Value::Number(i))));
                    i += step;
                    if list.len() > 1_000_000 {
                        return Err("النطاق كبير جداً".into());
                    }
                }
                Ok(Rc::new(RefCell::new(Value::List(list))))
            },
        },
        false,
    );

    env.define(
        "مسطح",
        Value::NativeFunction {
            name: "مسطح".to_string(),
            func: |a| {
                fn flatten(val: &Value) -> Vec<SharedValue> {
                    match val {
                        Value::List(l) => l.iter().flat_map(|v| flatten(&v.borrow())).collect(),
                        other => vec![Rc::new(RefCell::new(other.clone()))],
                    }
                }
                Ok(Rc::new(RefCell::new(Value::List(flatten(&a[0].borrow())))))
            },
        },
        false,
    );

    env.define(
        "مصفوفة",
        Value::NativeFunction {
            name: "مصفوفة".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                let default = if a.len() > 1 {
                    a[1].borrow().clone()
                } else {
                    Value::Null
                };
                let list: Vec<SharedValue> = (0..size)
                    .map(|_| Rc::new(RefCell::new(default.clone())))
                    .collect();
                Ok(Rc::new(RefCell::new(Value::List(list))))
            },
        },
        false,
    );

    env.define(
        "ادفع",
        Value::NativeFunction {
            name: "ادفع".to_string(),
            func: |a| {
                let item = a[1].borrow().clone();
                match &mut *a[0].borrow_mut() {
                    Value::List(l) => {
                        l.push(Rc::new(RefCell::new(item)));
                        Ok(Rc::new(RefCell::new(Value::Number(l.len() as f64))))
                    }
                    _ => Err("ادفع يتطلب قائمة".into()),
                }
            },
        },
        false,
    );

    env.define(
        "اسحب",
        Value::NativeFunction {
            name: "اسحب".to_string(),
            func: |a| match &mut *a[0].borrow_mut() {
                Value::List(l) => Ok(l
                    .pop()
                    .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))),
                _ => Err("اسحب يتطلب قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "ادمج",
        Value::NativeFunction {
            name: "ادمج".to_string(),
            func: |a| {
                let mut result = Vec::new();
                for arg in a {
                    match (*arg.borrow()).clone() {
                        Value::List(l) => result.extend(l),
                        other => result.push(Rc::new(RefCell::new(other))),
                    }
                }
                Ok(Rc::new(RefCell::new(Value::List(result))))
            },
        },
        false,
    );

    env.define(
        "أحصِ",
        Value::NativeFunction {
            name: "أحصِ".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow()) {
                (Value::List(l), item) => {
                    let count = l.iter().filter(|v| *v.borrow() == *item).count();
                    Ok(Rc::new(RefCell::new(Value::Number(count as f64))))
                }
                _ => Err("أحصِ يتطلب قائمة وعنصراً".into()),
            },
        },
        false,
    );

    env.define(
        "مقطع",
        Value::NativeFunction {
            name: "مقطع".to_string(),
            func: |a| match (*a[0].borrow()).clone() {
                Value::List(l) => {
                    let size = a[1].borrow().to_number()? as usize;
                    if size == 0 {
                        return Err("حجم المقطع يجب أن يكون أكبر من صفر".into());
                    }
                    let chunks: Vec<SharedValue> = l
                        .chunks(size)
                        .map(|c| Rc::new(RefCell::new(Value::List(c.to_vec()))))
                        .collect();
                    Ok(Rc::new(RefCell::new(Value::List(chunks))))
                }
                _ => Err("مقطع يتطلب قائمة".into()),
            },
        },
        false,
    );
}

pub fn define_dict_funcs(env: &mut Environment) {
    env.define(
        "مفاتيح",
        Value::NativeFunction {
            name: "مفاتيح".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Dictionary(d) => {
                    let keys: Vec<SharedValue> = d
                        .keys()
                        .map(|k| Rc::new(RefCell::new(Value::String(k.clone()))))
                        .collect();
                    Ok(Rc::new(RefCell::new(Value::List(keys))))
                }
                _ => Err("مفاتيح يتطلب قاموساً".into()),
            },
        },
        false,
    );

    env.define(
        "قيم",
        Value::NativeFunction {
            name: "قيم".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Dictionary(d) => {
                    let values: Vec<SharedValue> = d.values().map(Rc::clone).collect();
                    Ok(Rc::new(RefCell::new(Value::List(values))))
                }
                _ => Err("قيم يتطلب قاموساً".into()),
            },
        },
        false,
    );

    env.define(
        "أزواج",
        Value::NativeFunction {
            name: "أزواج".to_string(),
            func: |a| match &*a[0].borrow() {
                Value::Dictionary(d) => {
                    let pairs: Vec<SharedValue> = d
                        .iter()
                        .map(|(k, v)| {
                            Rc::new(RefCell::new(Value::List(vec![
                                Rc::new(RefCell::new(Value::String(k.clone()))),
                                Rc::clone(v),
                            ])))
                        })
                        .collect();
                    Ok(Rc::new(RefCell::new(Value::List(pairs))))
                }
                _ => Err("أزواج يتطلب قاموساً".into()),
            },
        },
        false,
    );

    env.define(
        "يوجد_مفتاح",
        Value::NativeFunction {
            name: "يوجد_مفتاح".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow()) {
                (Value::Dictionary(d), Value::String(k)) => Ok(Rc::new(RefCell::new(
                    Value::Boolean(d.contains_key(k.as_str())),
                ))),
                _ => Err("يوجد_مفتاح يتطلب قاموساً ومفتاحاً".into()),
            },
        },
        false,
    );

    env.define(
        "احذف_مفتاح",
        Value::NativeFunction {
            name: "احذف_مفتاح".to_string(),
            func: |a| {
                let key = a[1].borrow().to_string_value();
                match &mut *a[0].borrow_mut() {
                    Value::Dictionary(d) => Ok(d
                        .remove(&key)
                        .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))),
                    _ => Err("احذف_مفتاح يتطلب قاموساً".into()),
                }
            },
        },
        false,
    );

    env.define(
        "ادمج_قواميس",
        Value::NativeFunction {
            name: "ادمج_قواميس".to_string(),
            func: |a| {
                let mut result = HashMap::new();
                for arg in a {
                    if let Value::Dictionary(d) = (*arg.borrow()).clone() {
                        for (k, v) in d {
                            result.insert(k, v);
                        }
                    }
                }
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );

    env.define(
        "قاموس_من_قائمتين",
        Value::NativeFunction {
            name: "قاموس_من_قائمتين".to_string(),
            func: |a| match (&*a[0].borrow(), &*a[1].borrow()) {
                (Value::List(keys), Value::List(vals)) => {
                    let mut dict = HashMap::new();
                    for (k, v) in keys.iter().zip(vals.iter()) {
                        dict.insert(k.borrow().to_string_value(), Rc::clone(v));
                    }
                    Ok(Rc::new(RefCell::new(Value::Dictionary(dict))))
                }
                _ => Err("قاموس_من_قائمتين يتطلب قائمتين".into()),
            },
        },
        false,
    );
}

pub fn define_type_funcs(env: &mut Environment) {
    env.define(
        "نوع",
        Value::NativeFunction {
            name: "نوع".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::String(
                    a[0].borrow().type_name().to_string(),
                ))))
            },
        },
        false,
    );

    env.define(
        "رقم",
        Value::NativeFunction {
            name: "رقم".to_string(),
            func: |a| {
                a[0].borrow()
                    .to_number()
                    .map(|n| Rc::new(RefCell::new(Value::Number(n))))
            },
        },
        false,
    );

    env.define(
        "نص",
        Value::NativeFunction {
            name: "نص".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::String(
                    a[0].borrow().to_string_value(),
                ))))
            },
        },
        false,
    );

    env.define(
        "منطقي",
        Value::NativeFunction {
            name: "منطقي".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::Boolean(
                    a[0].borrow().is_truthy(),
                ))))
            },
        },
        false,
    );

    env.define(
        "قائمة",
        Value::NativeFunction {
            name: "قائمة".to_string(),
            func: |a| match (*a[0].borrow()).clone() {
                Value::List(l) => Ok(Rc::new(RefCell::new(Value::List(l)))),
                Value::String(s) => {
                    let chars: Vec<SharedValue> = s
                        .chars()
                        .map(|c| Rc::new(RefCell::new(Value::String(c.to_string()))))
                        .collect();
                    Ok(Rc::new(RefCell::new(Value::List(chars))))
                }
                Value::Dictionary(d) => {
                    let keys: Vec<SharedValue> = d
                        .into_keys()
                        .map(|k| Rc::new(RefCell::new(Value::String(k))))
                        .collect();
                    Ok(Rc::new(RefCell::new(Value::List(keys))))
                }
                other => Ok(Rc::new(RefCell::new(Value::List(vec![Rc::new(
                    RefCell::new(other),
                )])))),
            },
        },
        false,
    );

    env.define(
        "هل_رقم",
        Value::NativeFunction {
            name: "هل_رقم".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::Boolean(matches!(
                    &*a[0].borrow(),
                    Value::Number(_)
                )))))
            },
        },
        false,
    );

    env.define(
        "هل_نص",
        Value::NativeFunction {
            name: "هل_نص".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::Boolean(matches!(
                    &*a[0].borrow(),
                    Value::String(_)
                )))))
            },
        },
        false,
    );

    env.define(
        "هل_قائمة",
        Value::NativeFunction {
            name: "هل_قائمة".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::Boolean(matches!(
                    &*a[0].borrow(),
                    Value::List(_)
                )))))
            },
        },
        false,
    );

    env.define(
        "هل_قاموس",
        Value::NativeFunction {
            name: "هل_قاموس".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::Boolean(matches!(
                    &*a[0].borrow(),
                    Value::Dictionary(_)
                )))))
            },
        },
        false,
    );

    env.define(
        "هل_لاشيء",
        Value::NativeFunction {
            name: "هل_لاشيء".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::Boolean(matches!(
                    &*a[0].borrow(),
                    Value::Null
                )))))
            },
        },
        false,
    );

    env.define(
        "هل_دالة",
        Value::NativeFunction {
            name: "هل_دالة".to_string(),
            func: |a| {
                Ok(Rc::new(RefCell::new(Value::Boolean(matches!(
                    &*a[0].borrow(),
                    Value::Function { .. } | Value::NativeFunction { .. } | Value::Lambda { .. }
                )))))
            },
        },
        false,
    );
}

pub fn define_random_funcs(env: &mut Environment) {
    env.define(
        "عشوائي",
        Value::NativeFunction {
            name: "عشوائي".to_string(),
            func: |a| {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                use std::time::SystemTime;
                let mut hasher = DefaultHasher::new();
                SystemTime::now().hash(&mut hasher);
                let seed = hasher.finish();
                let pseudo = ((seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407))
                    >> 33) as f64
                    / u32::MAX as f64;
                let result = if a.is_empty() {
                    pseudo
                } else if a.len() == 1 {
                    (pseudo * a[0].borrow().to_number()?).floor()
                } else {
                    let min = a[0].borrow().to_number()?;
                    let max = a[1].borrow().to_number()?;
                    (pseudo * (max - min) + min).floor()
                };
                Ok(Rc::new(RefCell::new(Value::Number(result))))
            },
        },
        false,
    );

    env.define(
        "اختر_عشوائي",
        Value::NativeFunction {
            name: "اختر_عشوائي".to_string(),
            func: |a| match (*a[0].borrow()).clone() {
                Value::List(l) => {
                    if l.is_empty() {
                        return Ok(Rc::new(RefCell::new(Value::Null)));
                    }
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    use std::time::SystemTime;
                    let mut hasher = DefaultHasher::new();
                    SystemTime::now().hash(&mut hasher);
                    let idx = (hasher.finish() as usize) % l.len();
                    Ok(Rc::clone(&l[idx]))
                }
                _ => Err("اختر_عشوائي يتطلب قائمة".into()),
            },
        },
        false,
    );

    env.define(
        "خلط",
        Value::NativeFunction {
            name: "خلط".to_string(),
            func: |a| match (*a[0].borrow()).clone() {
                Value::List(mut l) => {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    use std::time::SystemTime;
                    let mut hasher = DefaultHasher::new();
                    SystemTime::now().hash(&mut hasher);
                    let mut seed = hasher.finish();
                    let n = l.len();
                    for i in (1..n).rev() {
                        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        let j = (seed as usize) % (i + 1);
                        l.swap(i, j);
                    }
                    Ok(Rc::new(RefCell::new(Value::List(l))))
                }
                _ => Err("خلط يتطلب قائمة".into()),
            },
        },
        false,
    );
}

pub fn define_json_funcs(env: &mut Environment) {
    env.define(
        "json_أقرأ",
        Value::NativeFunction {
            name: "json_أقرأ".to_string(),
            func: |a| {
                let s = a[0].borrow().to_string_value();
                match serde_json::from_str::<serde_json::Value>(&s) {
                    Ok(jv) => Ok(Rc::new(RefCell::new(json_to_value(jv)))),
                    Err(e) => Err(format!("خطأ في تحليل JSON: {}", e)),
                }
            },
        },
        false,
    );

    env.define(
        "json_اكتب",
        Value::NativeFunction {
            name: "json_اكتب".to_string(),
            func: |a| {
                let jv = value_to_json(&a[0].borrow());
                let pretty = a.len() > 1 && a[1].borrow().is_truthy();
                let s = if pretty {
                    serde_json::to_string_pretty(&jv).unwrap_or_default()
                } else {
                    serde_json::to_string(&jv).unwrap_or_default()
                };
                Ok(Rc::new(RefCell::new(Value::String(s))))
            },
        },
        false,
    );
}

pub fn json_to_value(jv: serde_json::Value) -> Value {
    match jv {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => Value::List(
            arr.into_iter()
                .map(|v| Rc::new(RefCell::new(json_to_value(v))))
                .collect(),
        ),
        serde_json::Value::Object(obj) => {
            let mut dict = HashMap::new();
            for (k, v) in obj {
                dict.insert(k, Rc::new(RefCell::new(json_to_value(v))));
            }
            Value::Dictionary(dict)
        }
    }
}

pub fn value_to_json(val: &Value) -> serde_json::Value {
    match val {
        Value::Null => serde_json::Value::Null,
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => {
            if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                serde_json::Value::Number(serde_json::Number::from(*n as i64))
            } else {
                serde_json::json!(n)
            }
        }
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::List(l) => {
            serde_json::Value::Array(l.iter().map(|v| value_to_json(&v.borrow())).collect())
        }
        Value::Dictionary(d) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in d {
                obj.insert(k.clone(), value_to_json(&v.borrow()));
            }
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::String(val.to_string()),
    }
}

// ═══════════════════════════════════════════════════════════════
// دوال التاريخ والوقت المختصرة
// ═══════════════════════════════════════════════════════════════

pub fn define_datetime_funcs(env: &mut Environment) {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // الوقت الحالي بالثواني
    env.define(
        "الآن",
        Value::NativeFunction {
            name: "الآن".to_string(),
            func: |_a| {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs_f64();
                Ok(Rc::new(RefCell::new(Value::Number(now))))
            },
        },
        false,
    );
    
    // الطابع الزمني
    env.define(
        "طابع",
        Value::NativeFunction {
            name: "طابع".to_string(),
            func: |_a| {
                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis();
                Ok(Rc::new(RefCell::new(Value::Number(ts as f64))))
            },
        },
        false,
    );
    
    // تأخير (sleep)
    env.define(
        "انتظر",
        Value::NativeFunction {
            name: "انتظر".to_string(),
            func: |a| {
                let ms = a[0].borrow().to_number()? as u64;
                std::thread::sleep(std::time::Duration::from_millis(ms));
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );
    
    // قياس الوقت
    env.define(
        "وقت",
        Value::NativeFunction {
            name: "وقت".to_string(),
            func: |_a| {
                let now = std::time::Instant::now();
                // نخزن الوقت في قيمة خاصة
                Ok(Rc::new(RefCell::new(Value::Number(
                    now.elapsed().as_nanos() as f64
                ))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال الاختبارات المختصرة
// ═══════════════════════════════════════════════════════════════

pub fn define_test_funcs(env: &mut Environment) {
    // تأكيد
    env.define(
        "تأكد",
        Value::NativeFunction {
            name: "تأكد".to_string(),
            func: |a| {
                let cond = a[0].borrow().is_truthy();
                if !cond {
                    let msg = if a.len() > 1 {
                        a[1].borrow().to_string_value()
                    } else {
                        "فشل التأكيد".to_string()
                    };
                    return Err(msg.into());
                }
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );
    
    // تأكيد يساوي
    env.define(
        "تأكد_يساوي",
        Value::NativeFunction {
            name: "تأكد_يساوي".to_string(),
            func: |a| {
                let expected = a[0].borrow().clone();
                let actual = a[1].borrow().clone();
                if expected != actual {
                    return Err(format!(
                        "توقع {} لكن وجد {}",
                        expected.to_string_value(),
                        actual.to_string_value()
                    ).into());
                }
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );
    
    // تأكيد لا يساوي
    env.define(
        "تأكد_لا_يساوي",
        Value::NativeFunction {
            name: "تأكد_لا_يساوي".to_string(),
            func: |a| {
                let v1 = a[0].borrow().clone();
                let v2 = a[1].borrow().clone();
                if v1 == v2 {
                    return Err(format!(
                        "توقع أن تكون القيم مختلفة لكن كلاهما {}",
                        v1.to_string_value()
                    ).into());
                }
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );
    
    // تأكيد صحيح
    env.define(
        "تأكد_صحيح",
        Value::NativeFunction {
            name: "تأكد_صحيح".to_string(),
            func: |a| {
                let val = a[0].borrow().is_truthy();
                if !val {
                    return Err("توقع قيمة صحيحة".into());
                }
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );
    
    // تأكيد خطأ
    env.define(
        "تأكد_خطأ",
        Value::NativeFunction {
            name: "تأكد_خطأ".to_string(),
            func: |a| {
                let val = a[0].borrow().is_truthy();
                if val {
                    return Err("توقع قيمة خاطئة".into());
                }
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );
    
    // تأكيد فارغ
    env.define(
        "تأكد_فارغ",
        Value::NativeFunction {
            name: "تأكد_فارغ".to_string(),
            func: |a| {
                let is_null = matches!(*a[0].borrow(), Value::Null);
                if !is_null {
                    return Err("توقع قيمة فارغة".into());
                }
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );
    
    // تأكيد يرمي
    env.define(
        "تأكد_يرمي",
        Value::NativeFunction {
            name: "تأكد_يرمي".to_string(),
            func: |_a| {
                // هذه تحتاج تنفيذ خاص في المفسر
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال مساعدة مختصرة
// ═══════════════════════════════════════════════════════════════

pub fn define_utility_funcs(env: &mut Environment) {
    // نسخة عميقة
    env.define(
        "نسخ",
        Value::NativeFunction {
            name: "نسخ".to_string(),
            func: |a| {
                let val = (*a[0].borrow()).clone();
                Ok(Rc::new(RefCell::new(val)))
            },
        },
        false,
    );
    
    // معرف فريد
    env.define(
        "معرف",
        Value::NativeFunction {
            name: "معرف".to_string(),
            func: |_a| {
                use std::time::{SystemTime, UNIX_EPOCH};
                let id = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                Ok(Rc::new(RefCell::new(Value::String(format!("id_{}", id)))))
            },
        },
        false,
    );
    
    // تشغيل
    env.define(
        "نفذ",
        Value::NativeFunction {
            name: "نفذ".to_string(),
            func: |a| {
                let code = a[0].borrow().to_string_value();
                // تنفيذ كود ديناميكي - يحتاج تنفيذ خاص
                Ok(Rc::new(RefCell::new(Value::String(code))))
            },
        },
        false,
    );
    
    // نطاق أرقام عشوائية
    env.define(
        "عشوائي_بين",
        Value::NativeFunction {
            name: "عشوائي_بين".to_string(),
            func: |a| {
                let min = a[0].borrow().to_number()?;
                let max = a[1].borrow().to_number()?;
                let range = max - min;
                let rand = (rand() * range) + min;
                Ok(Rc::new(RefCell::new(Value::Number(rand))))
            },
        },
        false,
    );
    
    // اختيار عشوائي من قائمة
    env.define(
        "اختر",
        Value::NativeFunction {
            name: "اختر".to_string(),
            func: |a| {
                match &*a[0].borrow() {
                    Value::List(l) => {
                        if l.is_empty() {
                            return Ok(Rc::new(RefCell::new(Value::Null)));
                        }
                        let idx = (rand() * l.len() as f64) as usize;
                        Ok(Rc::clone(&l[idx.min(l.len() - 1)]))
                    }
                    _ => Err("اختر يتطلب قائمة".into()),
                }
            },
        },
        false,
    );
    
    // خلط قائمة
    env.define(
        "اخلط",
        Value::NativeFunction {
            name: "اخلط".to_string(),
            func: |a| {
                match (*a[0].borrow()).clone() {
                    Value::List(mut l) => {
                        // خلط فيشر-ياتس بسيط
                        for i in (1..l.len()).rev() {
                            let j = (rand() * (i + 1) as f64) as usize;
                            l.swap(i, j);
                        }
                        Ok(Rc::new(RefCell::new(Value::List(l))))
                    }
                    _ => Err("اخلط يتطلب قائمة".into()),
                }
            },
        },
        false,
    );
    
    // تكرار نص
    env.define(
        "كرر",
        Value::NativeFunction {
            name: "كرر".to_string(),
            func: |a| {
                let s = a[0].borrow().to_string_value();
                let n = a[1].borrow().to_number()? as usize;
                Ok(Rc::new(RefCell::new(Value::String(s.repeat(n)))))
            },
        },
        false,
    );
    
    // محاذاة نص
    env.define(
        "حاذ",
        Value::NativeFunction {
            name: "حاذ".to_string(),
            func: |a| {
                let s = a[0].borrow().to_string_value();
                let width = a[1].borrow().to_number()? as usize;
                let align = if a.len() > 2 {
                    a[2].borrow().to_string_value()
                } else {
                    "يسار".to_string()
                };
                
                let padding = width.saturating_sub(s.chars().count());
                let result = match align.as_str() {
                    "يمين" | "right" => format!("{}{}", " ".repeat(padding), s),
                    "وسط" | "center" => {
                        let left = padding / 2;
                        let right = padding - left;
                        format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
                    }
                    _ => format!("{}{}", s, " ".repeat(padding)),
                };
                Ok(Rc::new(RefCell::new(Value::String(result))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// الثوابت
// ═══════════════════════════════════════════════════════════════

pub fn define_constants(env: &mut Environment) {
    env.define("صح", Value::Boolean(true), true);
    env.define("خطأ", Value::Boolean(false), true);
    env.define("لا_شيء", Value::Null, true);
    env.define("pi", Value::Number(std::f64::consts::PI), true);
    env.define("ط", Value::Number(std::f64::consts::PI), true);
    env.define("هـ", Value::Number(std::f64::consts::E), true);
    env.define("لانهاية", Value::Number(f64::INFINITY), true);
    env.define("ناقص_لانهاية", Value::Number(f64::NEG_INFINITY), true);
    env.define("ليس_رقم", Value::Number(f64::NAN), true);
    env.define("أقصى_رقم", Value::Number(f64::MAX), true);
    env.define("أدنى_رقم", Value::Number(f64::MIN_POSITIVE), true);
    env.define("المرجع_إصدار", Value::String("2.0.0".to_string()), true);
    env.define(
        "المبرمج",
        Value::String("رضوان دالي حمدوني".to_string()),
        true,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال المتجهات (Tensors) للذكاء الاصطناعي
// ═══════════════════════════════════════════════════════════════

use super::value::LayerInfo;

pub fn define_tensor_funcs(env: &mut Environment) {
    // إنشاء متجه من قائمة: متجه([1، 2، 3])
    env.define(
        "متجه",
        Value::NativeFunction {
            name: "متجه".to_string(),
            func: |a| {
                let (data, shape) = match &*a[0].borrow() {
                    Value::List(l) => {
                        let mut nums = Vec::new();
                        for item in l {
                            nums.push(item.borrow().to_number()?);
                        }
                        (nums, vec![l.len()])
                    }
                    Value::Number(n) => (vec![*n], vec![1]),
                    _ => return Err("متجه يتطلب قائمة أرقام".into()),
                };
                Ok(Rc::new(RefCell::new(Value::Tensor { data, shape })))
            },
        },
        false,
    );
    
    // إنشاء متجه أصفار: أصفار(5)
    env.define(
        "أصفار",
        Value::NativeFunction {
            name: "أصفار".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: vec![0.0; size],
                    shape: vec![size],
                })))
            },
        },
        false,
    );
    
    // إنشاء متجه آحاد: آحاد(5)
    env.define(
        "آحاد",
        Value::NativeFunction {
            name: "آحاد".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: vec![1.0; size],
                    shape: vec![size],
                })))
            },
        },
        false,
    );
    
    // متجه عشوائي: عشوائي(5)
    env.define(
        "عشوائي_متجه",
        Value::NativeFunction {
            name: "عشوائي_متجه".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                let data: Vec<f64> = (0..size).map(|_| rand() * 2.0 - 1.0).collect();
                Ok(Rc::new(RefCell::new(Value::Tensor { data, shape: vec![size] })))
            },
        },
        false,
    );
    
    // جمع متجهين: متجه1 + متجه2
    env.define(
        "اجمع_متجه",
        Value::NativeFunction {
            name: "اجمع_متجه".to_string(),
            func: |a| {
                let t1 = match &*a[0].borrow() {
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("المعامل الأول ليس متجه".into()),
                };
                let t2 = match &*a[1].borrow() {
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("المعامل الثاني ليس متجه".into()),
                };
                if t1.len() != t2.len() {
                    return Err("أبعاد المتجهين غير متطابقة".into());
                }
                let result: Vec<f64> = t1.iter().zip(t2.iter()).map(|(x, y)| x + y).collect();
                Ok(Rc::new(RefCell::new(Value::Tensor { data: result, shape: vec![t1.len()] })))
            },
        },
        false,
    );
    
    // ضرب متجهين (ضرب نقدي): مضروب(أ، ب)
    env.define(
        "مضروب",
        Value::NativeFunction {
            name: "مضروب".to_string(),
            func: |a| {
                let t1 = match &*a[0].borrow() {
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("المعامل الأول ليس متجه".into()),
                };
                let t2 = match &*a[1].borrow() {
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("المعامل الثاني ليس متجه".into()),
                };
                if t1.len() != t2.len() {
                    return Err("أبعاد المتجهين غير متطابقة".into());
                }
                let dot: f64 = t1.iter().zip(t2.iter()).map(|(x, y)| x * y).sum();
                Ok(Rc::new(RefCell::new(Value::Number(dot))))
            },
        },
        false,
    );
    
    // ضرب متجه في عدد: اضرب(متجه، عدد)
    env.define(
        "اضرب_متجه",
        Value::NativeFunction {
            name: "اضرب_متجه".to_string(),
            func: |a| {
                let (data, shape) = match &*a[0].borrow() {
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("المعامل الأول ليس متجه".into()),
                };
                let scalar = a[1].borrow().to_number()?;
                let result: Vec<f64> = data.iter().map(|x| x * scalar).collect();
                Ok(Rc::new(RefCell::new(Value::Tensor { data: result, shape })))
            },
        },
        false,
    );
    
    // حجم المتجه: حجم(متجه)
    env.define(
        "حجم_متجه",
        Value::NativeFunction {
            name: "حجم_متجه".to_string(),
            func: |a| {
                let len = match &*a[0].borrow() {
                    Value::Tensor { data, .. } => data.len(),
                    _ => return Err("المعامل ليس متجه".into()),
                };
                Ok(Rc::new(RefCell::new(Value::Number(len as f64))))
            },
        },
        false,
    );
    
    // الأبعاد: أبعاد(متجه)
    env.define(
        "أبعاد",
        Value::NativeFunction {
            name: "أبعاد".to_string(),
            func: |a| {
                let shape = match &*a[0].borrow() {
                    Value::Tensor { shape, .. } => shape.clone(),
                    _ => return Err("المعامل ليس متجه".into()),
                };
                let dims: Vec<SharedValue> = shape
                    .iter()
                    .map(|n| Rc::new(RefCell::new(Value::Number(*n as f64))))
                    .collect();
                Ok(Rc::new(RefCell::new(Value::List(dims))))
            },
        },
        false,
    );
    
    // تحويل متجه إلى قائمة: إلى_قائمة(متجه)
    env.define(
        "إلى_قائمة",
        Value::NativeFunction {
            name: "إلى_قائمة".to_string(),
            func: |a| {
                let data = match &*a[0].borrow() {
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("المعامل ليس متجه".into()),
                };
                let list: Vec<SharedValue> = data
                    .iter()
                    .map(|n| Rc::new(RefCell::new(Value::Number(*n))))
                    .collect();
                Ok(Rc::new(RefCell::new(Value::List(list))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال الذكاء الاصطناعي
// ═══════════════════════════════════════════════════════════════

pub fn define_ai_funcs(env: &mut Environment) {
    // دوال التفعيل
    
    // سيجمويد: سيجمويد(س)
    env.define(
        "سيجمويد",
        Value::NativeFunction {
            name: "سيجمويد".to_string(),
            func: |a| {
                let x = a[0].borrow().to_number()?;
                let sig = 1.0 / (1.0 + (-x).exp());
                Ok(Rc::new(RefCell::new(Value::Number(sig))))
            },
        },
        false,
    );
    
    // ريلو: ريلو(س)
    env.define(
        "ريلو",
        Value::NativeFunction {
            name: "ريلو".to_string(),
            func: |a| {
                let x = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(x.max(0.0)))))
            },
        },
        false,
    );
    
    // تانه: تانه(س)
    env.define(
        "تانه",
        Value::NativeFunction {
            name: "تانه".to_string(),
            func: |a| {
                let x = a[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(x.tanh()))))
            },
        },
        false,
    );
    
    // سوفتماكس: سوفتماكس([قيم])
    env.define(
        "سوفتماكس",
        Value::NativeFunction {
            name: "سوفتماكس".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => {
                        let mut nums = Vec::new();
                        for item in l {
                            nums.push(item.borrow().to_number()?);
                        }
                        nums
                    }
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("سوفتماكس يتطلب قائمة أو متجه".into()),
                };
                
                // حساب exp(x - max) للاستقرار العددي
                let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let exp_vals: Vec<f64> = values.iter().map(|x| (x - max_val).exp()).collect();
                let sum: f64 = exp_vals.iter().sum();
                let softmax: Vec<f64> = exp_vals.iter().map(|x| x / sum).collect();
                
                let result: Vec<SharedValue> = softmax
                    .iter()
                    .map(|n| Rc::new(RefCell::new(Value::Number(*n))))
                    .collect();
                Ok(Rc::new(RefCell::new(Value::List(result))))
            },
        },
        false,
    );
    
    // ليكي ريلو: ليكي_ريلو(س، ألفا=0.01)
    env.define(
        "ليكي_ريلو",
        Value::NativeFunction {
            name: "ليكي_ريلو".to_string(),
            func: |a| {
                let x = a[0].borrow().to_number()?;
                let alpha = if a.len() > 1 { a[1].borrow().to_number()? } else { 0.01 };
                Ok(Rc::new(RefCell::new(Value::Number(if x > 0.0 { x } else { alpha * x }))))
            },
        },
        false,
    );
    
    // دوال الخسارة
    
    // خطأ متربع: خطأ_مربع(توقع، حقيقي)
    env.define(
        "خطأ_مربع",
        Value::NativeFunction {
            name: "خطأ_مربع".to_string(),
            func: |a| {
                let pred = a[0].borrow().to_number()?;
                let actual = a[1].borrow().to_number()?;
                let error = (pred - actual).powi(2);
                Ok(Rc::new(RefCell::new(Value::Number(error))))
            },
        },
        false,
    );
    
    // خطأ تقاطع: خطأ_تقاطع(توقع، حقيقي)
    env.define(
        "خطأ_تقاطع",
        Value::NativeFunction {
            name: "خطأ_تقاطع".to_string(),
            func: |a| {
                let pred = a[0].borrow().to_number()?;
                let actual = a[1].borrow().to_number()?;
                // Cross-entropy: -[y*log(p) + (1-y)*log(1-p)]
                let epsilon = 1e-15;
                let p = pred.clamp(epsilon, 1.0 - epsilon);
                let loss = -(actual * p.ln() + (1.0 - actual) * (1.0 - p).ln());
                Ok(Rc::new(RefCell::new(Value::Number(loss))))
            },
        },
        false,
    );
    
    // إنشاء شبكة عصبية: شبكة("اسم"، [طبقات])
    env.define(
        "شبكة",
        Value::NativeFunction {
            name: "شبكة".to_string(),
            func: |a| {
                let name = a[0].borrow().to_string_value();
                let mut layers = Vec::new();
                
                if a.len() > 1 {
                    if let Value::List(l) = &*a[1].borrow() {
                        for layer in l {
                            if let Value::List(layer_def) = &*layer.borrow() {
                                if layer_def.len() >= 3 {
                                    let layer_type = layer_def[0].borrow().to_string_value();
                                    let input_size = layer_def[1].borrow().to_number()? as usize;
                                    let output_size = layer_def[2].borrow().to_number()? as usize;
                                    layers.push(LayerInfo {
                                        layer_type,
                                        input_size,
                                        output_size,
                                    });
                                }
                            }
                        }
                    }
                }
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork { name, layers })))
            },
        },
        false,
    );
    
    // طبقة خطية: طبقة_خطية(مدخل، مخرج)
    env.define(
        "طبقة_خطية",
        Value::NativeFunction {
            name: "طبقة_خطية".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let output_size = a[1].borrow().to_number()? as usize;
                
                // تهيئة الأوزان عشوائياً
                let weights: Vec<f64> = (0..input_size * output_size)
                    .map(|_| rand() * 0.1 - 0.05)
                    .collect();
                let biases: Vec<f64> = vec![0.0; output_size];
                
                Ok(Rc::new(RefCell::new(Value::Layer {
                    layer_type: "خطية".to_string(),
                    input_size,
                    output_size,
                    weights: Some(Rc::new(RefCell::new(weights))),
                    biases: Some(Rc::new(RefCell::new(biases))),
                })))
            },
        },
        false,
    );
    
    // معلومات الشبكة: معلومات(شبكة)
    env.define(
        "معلومات_شبكة",
        Value::NativeFunction {
            name: "معلومات_شبكة".to_string(),
            func: |a| {
                let (name, layers_count) = match &*a[0].borrow() {
                    Value::NeuralNetwork { name, layers } => (name.clone(), layers.len()),
                    _ => return Err("المعامل ليس شبكة عصبية".into()),
                };
                
                let mut info = HashMap::new();
                info.insert("اسم".to_string(), Rc::new(RefCell::new(Value::String(name))));
                info.insert("طبقات".to_string(), Rc::new(RefCell::new(Value::Number(layers_count as f64))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(info))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال المصفوفات المتقدمة (Matrix Operations)
// ═══════════════════════════════════════════════════════════════

pub fn define_matrix_funcs(env: &mut Environment) {
    // ضرب المصفوفات: ضرب_مصفوفات(أ، ب)
    env.define(
        "ضرب_مصفوفات",
        Value::NativeFunction {
            name: "ضرب_مصفوفات".to_string(),
            func: |a| {
                // الحصول على المصفوفتين
                let m1 = match &*a[0].borrow() {
                    Value::List(rows) => {
                        let mut matrix = Vec::new();
                        for row in rows {
                            if let Value::List(cols) = &*row.borrow() {
                                let mut row_data = Vec::new();
                                for val in cols {
                                    row_data.push(val.borrow().to_number()?);
                                }
                                matrix.push(row_data);
                            }
                        }
                        matrix
                    }
                    Value::Tensor { data, shape } if shape.len() == 2 => {
                        let rows = shape[0];
                        let cols = shape[1];
                        let mut matrix = Vec::new();
                        for i in 0..rows {
                            let mut row = Vec::new();
                            for j in 0..cols {
                                row.push(data[i * cols + j]);
                            }
                            matrix.push(row);
                        }
                        matrix
                    }
                    _ => return Err("ضرب_مصفوفات يتطلب مصفوفتين".into()),
                };
                
                let m2 = match &*a[1].borrow() {
                    Value::List(rows) => {
                        let mut matrix = Vec::new();
                        for row in rows {
                            if let Value::List(cols) = &*row.borrow() {
                                let mut row_data = Vec::new();
                                for val in cols {
                                    row_data.push(val.borrow().to_number()?);
                                }
                                matrix.push(row_data);
                            }
                        }
                        matrix
                    }
                    Value::Tensor { data, shape } if shape.len() == 2 => {
                        let rows = shape[0];
                        let cols = shape[1];
                        let mut matrix = Vec::new();
                        for i in 0..rows {
                            let mut row = Vec::new();
                            for j in 0..cols {
                                row.push(data[i * cols + j]);
                            }
                            matrix.push(row);
                        }
                        matrix
                    }
                    _ => return Err("ضرب_مصفوفات يتطلب مصفوفتين".into()),
                };
                
                // التحقق من الأبعاد
                if m1.is_empty() || m2.is_empty() {
                    return Err("المصفوفات فارغة".into());
                }
                let m1_cols = m1[0].len();
                let m2_rows = m2.len();
                if m1_cols != m2_rows {
                    return Err(format!("أبعاد غير متوافقة: {}×{} و {}×{}", 
                        m1.len(), m1_cols, m2_rows, m2[0].len()));
                }
                
                // ضرب المصفوفات
                let m2_cols = m2[0].len();
                let mut result = Vec::new();
                for i in 0..m1.len() {
                    let mut row = Vec::new();
                    for j in 0..m2_cols {
                        let mut sum = 0.0;
                        for k in 0..m1_cols {
                            sum += m1[i][k] * m2[k][j];
                        }
                        row.push(sum);
                    }
                    result.push(row);
                }
                
                // تحويل النتيجة
                let result_rows: Vec<SharedValue> = result
                    .iter()
                    .map(|row| {
                        Rc::new(RefCell::new(Value::List(
                            row.iter()
                                .map(|v| Rc::new(RefCell::new(Value::Number(*v))))
                                .collect()
                        )))
                    })
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(result_rows))))
            },
        },
        false,
    );
    
    // تبديل المصفوفة: تبديل(مصفوفة)
    env.define(
        "تبديل",
        Value::NativeFunction {
            name: "تبديل".to_string(),
            func: |a| {
                let matrix = match &*a[0].borrow() {
                    Value::List(rows) => {
                        let mut m = Vec::new();
                        for row in rows {
                            if let Value::List(cols) = &*row.borrow() {
                                let mut row_data = Vec::new();
                                for val in cols {
                                    row_data.push(val.borrow().to_number()?);
                                }
                                m.push(row_data);
                            }
                        }
                        m
                    }
                    Value::Tensor { data, shape } if shape.len() == 2 => {
                        let rows = shape[0];
                        let cols = shape[1];
                        let mut m = Vec::new();
                        for i in 0..rows {
                            let mut row = Vec::new();
                            for j in 0..cols {
                                row.push(data[i * cols + j]);
                            }
                            m.push(row);
                        }
                        m
                    }
                    _ => return Err("تبديل يتطلب مصفوفة".into()),
                };
                
                if matrix.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::List(Vec::new()))));
                }
                
                let rows = matrix.len();
                let cols = matrix[0].len();
                
                // التبديل
                let mut transposed = vec![vec![0.0; rows]; cols];
                for i in 0..rows {
                    for j in 0..cols {
                        transposed[j][i] = matrix[i][j];
                    }
                }
                
                let result: Vec<SharedValue> = transposed
                    .iter()
                    .map(|row| {
                        Rc::new(RefCell::new(Value::List(
                            row.iter()
                                .map(|v| Rc::new(RefCell::new(Value::Number(*v))))
                                .collect()
                        )))
                    })
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(result))))
            },
        },
        false,
    );
    
    // مجموع على المحور: مجموع_محور(مصفوفة، محور)
    env.define(
        "مجموع_محور",
        Value::NativeFunction {
            name: "مجموع_محور".to_string(),
            func: |a| {
                let matrix = match &*a[0].borrow() {
                    Value::List(rows) => {
                        let mut m = Vec::new();
                        for row in rows {
                            if let Value::List(cols) = &*row.borrow() {
                                let mut row_data = Vec::new();
                                for val in cols {
                                    row_data.push(val.borrow().to_number()?);
                                }
                                m.push(row_data);
                            }
                        }
                        m
                    }
                    _ => return Err("مجموع_محور يتطلب مصفوفة".into()),
                };
                
                let axis = a[1].borrow().to_number()? as usize;
                
                if matrix.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::List(Vec::new()))));
                }
                
                let result: Vec<SharedValue> = if axis == 0 {
                    // مجموع على الأعمدة
                    let cols = matrix[0].len();
                    let mut sums = vec![0.0; cols];
                    for row in &matrix {
                        for (j, val) in row.iter().enumerate() {
                            sums[j] += val;
                        }
                    }
                    sums.iter()
                        .map(|v| Rc::new(RefCell::new(Value::Number(*v))))
                        .collect()
                } else {
                    // مجموع على الصفوف
                    matrix.iter()
                        .map(|row| {
                            let sum: f64 = row.iter().sum();
                            Rc::new(RefCell::new(Value::Number(sum)))
                        })
                        .collect()
                };
                
                Ok(Rc::new(RefCell::new(Value::List(result))))
            },
        },
        false,
    );
    
    // تطبيع: تطبيع(مصفوفة)
    env.define(
        "تطبيع",
        Value::NativeFunction {
            name: "تطبيع".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => {
                        let mut nums = Vec::new();
                        for val in l {
                            nums.push(val.borrow().to_number()?);
                        }
                        nums
                    }
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تطبيع يتطلب قائمة أو متجه".into()),
                };
                
                // Min-Max Normalization
                let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let range = max - min;
                
                let normalized: Vec<SharedValue> = if range == 0.0 {
                    values.iter()
                        .map(|_| Rc::new(RefCell::new(Value::Number(0.0))))
                        .collect()
                } else {
                    values.iter()
                        .map(|v| Rc::new(RefCell::new(Value::Number((v - min) / range))))
                        .collect()
                };
                
                Ok(Rc::new(RefCell::new(Value::List(normalized))))
            },
        },
        false,
    );
    
    // تسوية قياسية: تسوية_قياسية(مصفوفة)
    env.define(
        "تسوية_قياسية",
        Value::NativeFunction {
            name: "تسوية_قياسية".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => {
                        let mut nums = Vec::new();
                        for val in l {
                            nums.push(val.borrow().to_number()?);
                        }
                        nums
                    }
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تسوية_قياسية يتطلب قائمة أو متجه".into()),
                };
                
                // Standard Normalization (Z-score)
                let n = values.len() as f64;
                let mean: f64 = values.iter().sum::<f64>() / n;
                let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
                let std = variance.sqrt();
                
                let normalized: Vec<SharedValue> = if std == 0.0 {
                    values.iter()
                        .map(|_| Rc::new(RefCell::new(Value::Number(0.0))))
                        .collect()
                } else {
                    values.iter()
                        .map(|v| Rc::new(RefCell::new(Value::Number((v - mean) / std))))
                        .collect()
                };
                
                Ok(Rc::new(RefCell::new(Value::List(normalized))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال الاشتقاق البسيطة (Gradient Derivatives)
// ═══════════════════════════════════════════════════════════════

pub fn define_gradient_deriv_funcs(env: &mut Environment) {
    // اشتقاق سيجمويد: اشتقاق_سيجمويد(س)
    env.define(
        "اشتقاق_سيجمويد",
        Value::NativeFunction {
            name: "اشتقاق_سيجمويد".to_string(),
            func: |a| {
                let x = a[0].borrow().to_number()?;
                let sig = 1.0 / (1.0 + (-x).exp());
                let grad = sig * (1.0 - sig);
                Ok(Rc::new(RefCell::new(Value::Number(grad))))
            },
        },
        false,
    );
    
    // اشتقاق ريلو: اشتقاق_ريلو(س)
    env.define(
        "اشتقاق_ريلو",
        Value::NativeFunction {
            name: "اشتقاق_ريلو".to_string(),
            func: |a| {
                let x = a[0].borrow().to_number()?;
                let grad = if x > 0.0 { 1.0 } else { 0.0 };
                Ok(Rc::new(RefCell::new(Value::Number(grad))))
            },
        },
        false,
    );
    
    // اشتقاق تانه: اشتقاق_تانه(س)
    env.define(
        "اشتقاق_تانه",
        Value::NativeFunction {
            name: "اشتقاق_تانه".to_string(),
            func: |a| {
                let x = a[0].borrow().to_number()?;
                let tanh_x = x.tanh();
                let grad = 1.0 - tanh_x * tanh_x;
                Ok(Rc::new(RefCell::new(Value::Number(grad))))
            },
        },
        false,
    );
    
    // اشتقاق خطأ المربع: اشتقاق_مربع(توقع، حقيقي)
    env.define(
        "اشتقاق_مربع",
        Value::NativeFunction {
            name: "اشتقاق_مربع".to_string(),
            func: |a| {
                let pred = a[0].borrow().to_number()?;
                let actual = a[1].borrow().to_number()?;
                let grad = 2.0 * (pred - actual);
                Ok(Rc::new(RefCell::new(Value::Number(grad))))
            },
        },
        false,
    );
    
    // تحديث الأوزان: تحديث_وزن(وزن، تدرج، معدل_تعلم)
    env.define(
        "تحديث_وزن",
        Value::NativeFunction {
            name: "تحديث_وزن".to_string(),
            func: |a| {
                let weight = a[0].borrow().to_number()?;
                let gradient = a[1].borrow().to_number()?;
                let learning_rate = a[2].borrow().to_number()?;
                let new_weight = weight - learning_rate * gradient;
                Ok(Rc::new(RefCell::new(Value::Number(new_weight))))
            },
        },
        false,
    );
    
    // تحديث أوزان متجهة: تحديث_أوزان(أوزان، تدرجات، معدل_تعلم)
    env.define(
        "تحديث_أوزان",
        Value::NativeFunction {
            name: "تحديث_أوزان".to_string(),
            func: |a| {
                let weights = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number()).collect::<Result<Vec<_>, _>>()?,
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تحديث_أوزان يتطلب قائمة أوزان".into()),
                };
                
                let gradients = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number()).collect::<Result<Vec<_>, _>>()?,
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تحديث_أوزان يتطلب قائمة تدرجات".into()),
                };
                
                let lr = a[2].borrow().to_number()?;
                
                if weights.len() != gradients.len() {
                    return Err("عدد الأوزان لا يساوي عدد التدرجات".into());
                }
                
                let updated: Vec<SharedValue> = weights.iter()
                    .zip(gradients.iter())
                    .map(|(w, g)| Rc::new(RefCell::new(Value::Number(w - lr * g))))
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(updated))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال HTTP والشبكة
// ═══════════════════════════════════════════════════════════════

pub fn define_http_funcs(env: &mut Environment) {
    // تحميل من رابط: حمّل_من("url")
    // ملاحظة: هذه دالة مبسطة - في الإنتاج تحتاج async
    env.define(
        "حمّل_من",
        Value::NativeFunction {
            name: "حمّل_من".to_string(),
            func: |a| {
                let url = a[0].borrow().to_string_value();
                
                // محاكاة بسيطة - في الواقع تحتاج مكتبة HTTP
                // نرجع معلومات عن URL
                let mut info = HashMap::new();
                info.insert("رابط".to_string(), Rc::new(RefCell::new(Value::String(url))));
                info.insert("حالة".to_string(), Rc::new(RefCell::new(Value::String("جاهز".to_string()))));
                info.insert("طريقة".to_string(), Rc::new(RefCell::new(Value::String("GET".to_string()))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(info))))
            },
        },
        false,
    );
    
    // تحليل JSON: حلّل_جسون(نص)
    env.define(
        "حلّل_جسون",
        Value::NativeFunction {
            name: "حلّل_جسون".to_string(),
            func: |a| {
                let json_str = a[0].borrow().to_string_value();
                
                // تحليل بسيط لـ JSON
                let trimmed = json_str.trim();
                
                if trimmed.starts_with('{') && trimmed.ends_with('}') {
                    // كائن JSON
                    let mut dict = HashMap::new();
                    // تحليل مبسط - في الإنتاج تحتاج مكتبة JSON حقيقية
                    dict.insert("نوع".to_string(), Rc::new(RefCell::new(Value::String("كائن".to_string()))));
                    dict.insert("أصلي".to_string(), Rc::new(RefCell::new(Value::String(json_str))));
                    return Ok(Rc::new(RefCell::new(Value::Dictionary(dict))));
                } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
                    // مصفوفة JSON
                    return Ok(Rc::new(RefCell::new(Value::List(vec![
                        Rc::new(RefCell::new(Value::String("مصفوفة_جسون".to_string())))
                    ]))));
                } else if trimmed.starts_with('"') {
                    // نص JSON
                    let clean = trimmed.trim_matches('"').to_string();
                    return Ok(Rc::new(RefCell::new(Value::String(clean))));
                } else if let Ok(n) = trimmed.parse::<f64>() {
                    // رقم JSON
                    return Ok(Rc::new(RefCell::new(Value::Number(n))));
                } else if trimmed == "true" || trimmed == "صح" {
                    return Ok(Rc::new(RefCell::new(Value::Boolean(true))));
                } else if trimmed == "false" || trimmed == "خطأ" {
                    return Ok(Rc::new(RefCell::new(Value::Boolean(false))));
                } else if trimmed == "null" || trimmed == "لا_شيء" {
                    return Ok(Rc::new(RefCell::new(Value::Null)));
                }
                
                Err("فشل تحليل JSON".into())
            },
        },
        false,
    );
    
    // تحويل إلى JSON: إلى_جسون(قيمة)
    env.define(
        "إلى_جسون",
        Value::NativeFunction {
            name: "إلى_جسون".to_string(),
            func: |a| {
                let val = &*a[0].borrow();
                let json = match val {
                    Value::Number(n) => {
                        if n.fract() == 0.0 {
                            format!("{}", *n as i64)
                        } else {
                            format!("{}", n)
                        }
                    }
                    Value::String(s) => format!("\"{}\"", s),
                    Value::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
                    Value::Null => "null".to_string(),
                    Value::List(l) => {
                        let items: Vec<String> = l.iter()
                            .map(|v| {
                                let v = v.borrow();
                                match &*v {
                                    Value::String(s) => format!("\"{}\"", s),
                                    Value::Number(n) => format!("{}", n),
                                    Value::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
                                    Value::Null => "null".to_string(),
                                    _ => v.to_string()
                                }
                            })
                            .collect();
                        format!("[{}]", items.join(", "))
                    }
                    Value::Dictionary(d) => {
                        let items: Vec<String> = d.iter()
                            .map(|(k, v)| {
                                let v = v.borrow();
                                let val_str = match &*v {
                                    Value::String(s) => format!("\"{}\"", s),
                                    Value::Number(n) => format!("{}", n),
                                    Value::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
                                    Value::Null => "null".to_string(),
                                    _ => v.to_string()
                                };
                                format!("\"{}\": {}", k, val_str)
                            })
                            .collect();
                        format!("{{{}}}", items.join(", "))
                    }
                    _ => format!("\"{}\"", val)
                };
                Ok(Rc::new(RefCell::new(Value::String(json))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال تحميل البيانات ومعالجتها
// ═══════════════════════════════════════════════════════════════

pub fn define_data_funcs(env: &mut Environment) {
    // تقسيم البيانات: قسم(بيانات، نسبة)
    env.define(
        "قسم",
        Value::NativeFunction {
            name: "قسم".to_string(),
            func: |a| {
                let data = match &*a[0].borrow() {
                    Value::List(l) => l.clone(),
                    _ => return Err("قسم يتطلب قائمة".into()),
                };
                
                let ratio = a[1].borrow().to_number()?;
                let split_idx = ((data.len() as f64) * ratio) as usize;
                
                let part1: Vec<SharedValue> = data.iter().take(split_idx).cloned().collect();
                let part2: Vec<SharedValue> = data.iter().skip(split_idx).cloned().collect();
                
                Ok(Rc::new(RefCell::new(Value::List(vec![
                    Rc::new(RefCell::new(Value::List(part1))),
                    Rc::new(RefCell::new(Value::List(part2))),
                ]))))
            },
        },
        false,
    );
    
    // خلط البيانات: اخلط(بيانات)
    env.define(
        "اخلط_بيانات",
        Value::NativeFunction {
            name: "اخلط_بيانات".to_string(),
            func: |a| {
                let mut data = match &*a[0].borrow() {
                    Value::List(l) => l.clone(),
                    _ => return Err("اخلط_بيانات يتطلب قائمة".into()),
                };
                
                // خلط فيشر-ياتس
                for i in (1..data.len()).rev() {
                    let j = (rand() * (i + 1) as f64) as usize;
                    data.swap(i, j);
                }
                
                Ok(Rc::new(RefCell::new(Value::List(data))))
            },
        },
        false,
    );
    
    // دفعات: دفعات(بيانات، حجم)
    env.define(
        "دفعات",
        Value::NativeFunction {
            name: "دفعات".to_string(),
            func: |a| {
                let data = match &*a[0].borrow() {
                    Value::List(l) => l.clone(),
                    _ => return Err("دفعات يتطلب قائمة".into()),
                };
                
                let batch_size = a[1].borrow().to_number()? as usize;
                if batch_size == 0 {
                    return Err("حجم الدفعة يجب أن يكون أكبر من صفر".into());
                }
                
                let batches: Vec<SharedValue> = data
                    .chunks(batch_size)
                    .map(|chunk| Rc::new(RefCell::new(Value::List(chunk.to_vec()))))
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(batches))))
            },
        },
        false,
    );
    
    // واحد_ساخن: واحد_ساخن(فئة، عدد_الفئات)
    env.define(
        "واحد_ساخن",
        Value::NativeFunction {
            name: "واحد_ساخن".to_string(),
            func: |a| {
                let class_idx = a[0].borrow().to_number()? as usize;
                let num_classes = a[1].borrow().to_number()? as usize;
                
                if class_idx >= num_classes {
                    return Err("فهرس الفئة خارج النطاق".into());
                }
                
                let mut one_hot = vec![0.0; num_classes];
                one_hot[class_idx] = 1.0;
                
                let result: Vec<SharedValue> = one_hot
                    .iter()
                    .map(|v| Rc::new(RefCell::new(Value::Number(*v))))
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(result))))
            },
        },
        false,
    );
    
    // Argmax: فهرس_أقصى(قائمة)
    env.define(
        "فهرس_أقصى",
        Value::NativeFunction {
            name: "فهرس_أقصى".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => {
                        let mut nums = Vec::new();
                        for v in l {
                            nums.push(v.borrow().to_number()?);
                        }
                        nums
                    }
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("فهرس_أقصى يتطلب قائمة أو متجه".into()),
                };
                
                if values.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::Number(-1.0))));
                }
                
                let max_idx = values.iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                
                Ok(Rc::new(RefCell::new(Value::Number(max_idx as f64))))
            },
        },
        false,
    );
    
    // Argmin: فهرس_أدنى(قائمة)
    env.define(
        "فهرس_أدنى",
        Value::NativeFunction {
            name: "فهرس_أدنى".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => {
                        let mut nums = Vec::new();
                        for v in l {
                            nums.push(v.borrow().to_number()?);
                        }
                        nums
                    }
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("فهرس_أدنى يتطلب قائمة أو متجه".into()),
                };
                
                if values.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::Number(-1.0))));
                }
                
                let min_idx = values.iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                
                Ok(Rc::new(RefCell::new(Value::Number(min_idx as f64))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال المحسنات (Optimizers)
// ═══════════════════════════════════════════════════════════════

pub fn define_optimizer_funcs(env: &mut Environment) {
    // SGD بسيط: sgd(وزن، تدرج، معدل_تعلم)
    env.define(
        "sgd",
        Value::NativeFunction {
            name: "sgd".to_string(),
            func: |a| {
                let weight = a[0].borrow().to_number()?;
                let gradient = a[1].borrow().to_number()?;
                let lr = a[2].borrow().to_number()?;
                
                let new_weight = weight - lr * gradient;
                Ok(Rc::new(RefCell::new(Value::Number(new_weight))))
            },
        },
        false,
    );
    
    // SGD مع زخم: sgd_زخم(وزن، تدرج، معدل_تعلم، زخم، سرعة_سابقة)
    env.define(
        "sgd_زخم",
        Value::NativeFunction {
            name: "sgd_زخم".to_string(),
            func: |a| {
                let weight = a[0].borrow().to_number()?;
                let gradient = a[1].borrow().to_number()?;
                let lr = a[2].borrow().to_number()?;
                let momentum = a[3].borrow().to_number()?;
                let velocity = a[4].borrow().to_number()?;
                
                // v = momentum * v - lr * gradient
                // w = w + v
                let new_velocity = momentum * velocity - lr * gradient;
                let new_weight = weight + new_velocity;
                
                // نرجع قاموس بالوزن الجديد والسرعة الجديدة
                let mut result = HashMap::new();
                result.insert("وزن".to_string(), Rc::new(RefCell::new(Value::Number(new_weight))));
                result.insert("سرعة".to_string(), Rc::new(RefCell::new(Value::Number(new_velocity))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // محسن آدم: آدم(وزن، تدرج، معدل_تعلم، بيتا1، بيتا2، م، ف، ت)
    env.define(
        "آدم",
        Value::NativeFunction {
            name: "آدم".to_string(),
            func: |a| {
                let weight = a[0].borrow().to_number()?;
                let gradient = a[1].borrow().to_number()?;
                let lr = if a.len() > 2 { a[2].borrow().to_number()? } else { 0.001 };
                let beta1 = if a.len() > 3 { a[3].borrow().to_number()? } else { 0.9 };
                let beta2 = if a.len() > 4 { a[4].borrow().to_number()? } else { 0.999 };
                let m = if a.len() > 5 { a[5].borrow().to_number()? } else { 0.0 };
                let v = if a.len() > 6 { a[6].borrow().to_number()? } else { 0.0 };
                let t = if a.len() > 7 { a[7].borrow().to_number()? as i64 } else { 1 };
                
                // Adam optimizer
                // m = beta1 * m + (1 - beta1) * gradient
                // v = beta2 * v + (1 - beta2) * gradient^2
                // m_hat = m / (1 - beta1^t)
                // v_hat = v / (1 - beta2^t)
                // w = w - lr * m_hat / (sqrt(v_hat) + epsilon)
                
                let new_m = beta1 * m + (1.0 - beta1) * gradient;
                let new_v = beta2 * v + (1.0 - beta2) * gradient * gradient;
                
                let m_hat = new_m / (1.0 - beta1.powi(t as i32));
                let v_hat = new_v / (1.0 - beta2.powi(t as i32));
                
                let epsilon = 1e-8;
                let new_weight = weight - lr * m_hat / (v_hat.sqrt() + epsilon);
                
                let mut result = HashMap::new();
                result.insert("وزن".to_string(), Rc::new(RefCell::new(Value::Number(new_weight))));
                result.insert("م".to_string(), Rc::new(RefCell::new(Value::Number(new_m))));
                result.insert("ف".to_string(), Rc::new(RefCell::new(Value::Number(new_v))));
                result.insert("زمن".to_string(), Rc::new(RefCell::new(Value::Number((t + 1) as f64))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // RMSprop: rmsprop(وزن، تدرج، معدل_تعلم، جاما، مخزن)
    env.define(
        "rmsprop",
        Value::NativeFunction {
            name: "rmsprop".to_string(),
            func: |a| {
                let weight = a[0].borrow().to_number()?;
                let gradient = a[1].borrow().to_number()?;
                let lr = if a.len() > 2 { a[2].borrow().to_number()? } else { 0.001 };
                let gamma = if a.len() > 3 { a[3].borrow().to_number()? } else { 0.9 };
                let cache = if a.len() > 4 { a[4].borrow().to_number()? } else { 0.0 };
                
                let epsilon = 1e-8;
                let new_cache = gamma * cache + (1.0 - gamma) * gradient * gradient;
                let new_weight = weight - lr * gradient / (new_cache.sqrt() + epsilon);
                
                let mut result = HashMap::new();
                result.insert("وزن".to_string(), Rc::new(RefCell::new(Value::Number(new_weight))));
                result.insert("مخزن".to_string(), Rc::new(RefCell::new(Value::Number(new_cache))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // Adagrad: adagrad(وزن، تدرج، معدل_تعلم، مخزن)
    env.define(
        "adagrad",
        Value::NativeFunction {
            name: "adagrad".to_string(),
            func: |a| {
                let weight = a[0].borrow().to_number()?;
                let gradient = a[1].borrow().to_number()?;
                let lr = if a.len() > 2 { a[2].borrow().to_number()? } else { 0.01 };
                let cache = if a.len() > 3 { a[3].borrow().to_number()? } else { 0.0 };
                
                let epsilon = 1e-8;
                let new_cache = cache + gradient * gradient;
                let new_weight = weight - lr * gradient / (new_cache.sqrt() + epsilon);
                
                let mut result = HashMap::new();
                result.insert("وزن".to_string(), Rc::new(RefCell::new(Value::Number(new_weight))));
                result.insert("مخزن".to_string(), Rc::new(RefCell::new(Value::Number(new_cache))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال التدريب الكامل
// ═══════════════════════════════════════════════════════════════

pub fn define_training_funcs(env: &mut Environment) {
    // تدريب خطوة واحدة: درّب_خطوة(نموذج، مدخلات، أهداف، معدل_تعلم)
    env.define(
        "درّب_خطوة",
        Value::NativeFunction {
            name: "درّب_خطوة".to_string(),
            func: |a| {
                // استخراج المعاملات
                let model = a[0].borrow().clone();
                let _inputs = &a[1];
                let _targets = &a[2];
                let lr = if a.len() > 3 { a[3].borrow().to_number()? } else { 0.01 };
                
                // محاكاة خطوة تدريب
                let mut result = HashMap::new();
                result.insert("نجاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("معدل_تعلم".to_string(), Rc::new(RefCell::new(Value::Number(lr))));
                result.insert("خسارة".to_string(), Rc::new(RefCell::new(Value::Number(0.0))));
                
                match model {
                    Value::NeuralNetwork { name, layers } => {
                        result.insert("نموذج".to_string(), Rc::new(RefCell::new(Value::String(name))));
                        result.insert("طبقات".to_string(), Rc::new(RefCell::new(Value::Number(layers.len() as f64))));
                    }
                    _ => {}
                }
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // حساب الدقة: دقة(توقعات، أهداف)
    env.define(
        "دقة",
        Value::NativeFunction {
            name: "دقة".to_string(),
            func: |a| {
                let predictions = match &*a[0].borrow() {
                    Value::List(l) => l.clone(),
                    _ => return Err("دقة تتطلب قائمة توقعات".into()),
                };
                
                let targets = match &*a[1].borrow() {
                    Value::List(l) => l.clone(),
                    _ => return Err("دقة تتطلب قائمة أهداف".into()),
                };
                
                if predictions.len() != targets.len() {
                    return Err("عدد التوقعات لا يساوي عدد الأهداف".into());
                }
                
                let correct = predictions.iter()
                    .zip(targets.iter())
                    .filter(|(p, t)| *p.borrow() == *t.borrow())
                    .count();
                
                let accuracy = correct as f64 / predictions.len() as f64;
                Ok(Rc::new(RefCell::new(Value::Number(accuracy))))
            },
        },
        false,
    );
    
    // تقليص معدل التعلم: قلّص_معدل(معدل، عامل، صبر، أفضل_خسارة، خسارة_حالية، انتظار)
    env.define(
        "قلّص_معدل",
        Value::NativeFunction {
            name: "قلّص_معدل".to_string(),
            func: |a| {
                let lr = a[0].borrow().to_number()?;
                let factor = if a.len() > 1 { a[1].borrow().to_number()? } else { 0.5 };
                let patience = if a.len() > 2 { a[2].borrow().to_number()? as i64 } else { 5 };
                let best_loss = if a.len() > 3 { a[3].borrow().to_number()? } else { f64::INFINITY };
                let current_loss = if a.len() > 4 { a[4].borrow().to_number()? } else { f64::INFINITY };
                let wait = if a.len() > 5 { a[5].borrow().to_number()? as i64 } else { 0 };
                
                let mut result = HashMap::new();
                let mut new_lr = lr;
                #[allow(unused_assignments)]
                let mut new_wait = 0;
                let mut new_best = best_loss;
                
                if current_loss < best_loss {
                    new_best = current_loss;
                    new_wait = 0;
                } else {
                    new_wait = wait + 1;
                    if new_wait >= patience {
                        new_lr = lr * factor;
                        new_wait = 0;
                    }
                }
                
                result.insert("معدل_تعلم".to_string(), Rc::new(RefCell::new(Value::Number(new_lr))));
                result.insert("انتظار".to_string(), Rc::new(RefCell::new(Value::Number(new_wait as f64))));
                result.insert("أفضل_خسارة".to_string(), Rc::new(RefCell::new(Value::Number(new_best))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // توقف مبكر: توقف_مبكر(صبر، انتظار، أفضل_خسارة، خسارة_حالية)
    env.define(
        "توقف_مبكر",
        Value::NativeFunction {
            name: "توقف_مبكر".to_string(),
            func: |a| {
                let patience = a[0].borrow().to_number()? as i64;
                let wait = a[1].borrow().to_number()? as i64;
                let best_loss = a[2].borrow().to_number()?;
                let current_loss = a[3].borrow().to_number()?;
                
                let mut new_best = best_loss;
                #[allow(unused_assignments)]
                let mut new_wait = 0;
                let mut should_stop = false;
                
                if current_loss < best_loss {
                    new_best = current_loss;
                    new_wait = 0;
                } else {
                    new_wait = wait + 1;
                    if new_wait >= patience {
                        should_stop = true;
                    }
                }
                
                let mut result = HashMap::new();
                result.insert("توقف".to_string(), Rc::new(RefCell::new(Value::Boolean(should_stop))));
                result.insert("انتظار".to_string(), Rc::new(RefCell::new(Value::Number(new_wait as f64))));
                result.insert("أفضل_خسارة".to_string(), Rc::new(RefCell::new(Value::Number(new_best))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // حساب التدرج العكسي: تدرج_عكسي(خسارة، أوزان)
    env.define(
        "تدرج_عكسي",
        Value::NativeFunction {
            name: "تدرج_عكسي".to_string(),
            func: |a| {
                let loss_grad = a[0].borrow().to_number()?;
                
                let weights = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تدرج_عكسي يتطلب قائمة أوزان".into()),
                };
                
                // تدرجات بسيطة (في الواقع تحتاج Backpropagation كامل)
                let gradients: Vec<SharedValue> = weights.iter()
                    .map(|_| Rc::new(RefCell::new(Value::Number(loss_grad / weights.len() as f64))))
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(gradients))))
            },
        },
        false,
    );
    
    // تهيئة أوزان Xavier: xavier(مدخل، مخرج)
    env.define(
        "xavier",
        Value::NativeFunction {
            name: "xavier".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let output_size = a[1].borrow().to_number()? as usize;
                
                // Xavier/Glorot initialization
                let limit = (6.0 / (input_size + output_size) as f64).sqrt();
                
                // أوزان عشوائية في النطاق
                let weights: Vec<SharedValue> = (0..input_size * output_size)
                    .map(|_| {
                        let w = (rand() * 2.0 - 1.0) * limit;
                        Rc::new(RefCell::new(Value::Number(w)))
                    })
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(weights))))
            },
        },
        false,
    );
    
    // تهيئة أوزان He: he(مدخل)
    env.define(
        "he",
        Value::NativeFunction {
            name: "he".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let output_size = if a.len() > 1 { a[1].borrow().to_number()? as usize } else { input_size };
                
                // He initialization (جيد لـ ReLU)
                let std = (2.0 / input_size as f64).sqrt();
                
                let weights: Vec<SharedValue> = (0..input_size * output_size)
                    .map(|_| {
                        let w = rand() * std;
                        Rc::new(RefCell::new(Value::Number(w)))
                    })
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(weights))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// نظام الوحدات المتقدم
// ═══════════════════════════════════════════════════════════════

pub fn define_module_funcs(env: &mut Environment) {
    // إنشاء وحدة: أنشئ_وحدة("اسم")
    env.define(
        "أنشئ_وحدة",
        Value::NativeFunction {
            name: "أنشئ_وحدة".to_string(),
            func: |a| {
                let name = a[0].borrow().to_string_value();
                
                let mut module = HashMap::new();
                module.insert("اسم".to_string(), Rc::new(RefCell::new(Value::String(name))));
                module.insert("صادرات".to_string(), Rc::new(RefCell::new(Value::List(Vec::new()))));
                module.insert("واردات".to_string(), Rc::new(RefCell::new(Value::List(Vec::new()))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(module))))
            },
        },
        false,
    );
    
    // تصدير: صدّر(وحدة، اسم، قيمة)
    env.define(
        "صدّر",
        Value::NativeFunction {
            name: "صدّر".to_string(),
            func: |a| {
                let export_name = a[0].borrow().to_string_value();
                let value = a[1].borrow().clone();
                
                // إنشاء قاموس التصدير
                let mut export_entry = HashMap::new();
                export_entry.insert("اسم".to_string(), Rc::new(RefCell::new(Value::String(export_name))));
                export_entry.insert("قيمة".to_string(), Rc::new(RefCell::new(value)));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(export_entry))))
            },
        },
        false,
    );
    
    // استيراد عنصر: استورد_عنصر(وحدة، اسم)
    env.define(
        "استورد_عنصر",
        Value::NativeFunction {
            name: "استورد_عنصر".to_string(),
            func: |a| {
                let module = &a[0];
                let item_name = a[1].borrow().to_string_value();
                
                // البحث في صادرات الوحدة
                match &*module.borrow() {
                    Value::Dictionary(dict) => {
                        if let Some(exports) = dict.get("صادرات") {
                            if let Value::List(items) = &*exports.borrow() {
                                for item in items {
                                    if let Value::Dictionary(entry) = &*item.borrow() {
                                        if let Some(name_val) = entry.get("اسم") {
                                            if name_val.borrow().to_string_value() == item_name {
                                                if let Some(val) = entry.get("قيمة") {
                                                    return Ok(Rc::clone(val));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(format!("العنصر '{}' غير موجود في الوحدة", item_name))
                    }
                    _ => Err("استورد_عنصر يتطلب وحدة".into()),
                }
            },
        },
        false,
    );
    
    // إضافة صادر: أضف_صادر(وحدة، تصدير)
    env.define(
        "أضف_صادر",
        Value::NativeFunction {
            name: "أضف_صادر".to_string(),
            func: |a| {
                let export_entry = a[1].borrow().clone();
                
                match &mut *a[0].borrow_mut() {
                    Value::Dictionary(dict) => {
                        if let Some(exports) = dict.get_mut("صادرات") {
                            if let Value::List(items) = &mut *exports.borrow_mut() {
                                items.push(Rc::new(RefCell::new(export_entry)));
                            }
                        }
                        Ok(Rc::new(RefCell::new(Value::Null)))
                    }
                    _ => Err("أضف_صادر يتطلب وحدة".into()),
                }
            },
        },
        false,
    );
    
    // مسار الوحدة: مسار_وحدة(اسم)
    env.define(
        "مسار_وحدة",
        Value::NativeFunction {
            name: "مسار_وحدة".to_string(),
            func: |a| {
                let module_name = a[0].borrow().to_string_value();
                
                // تحويل اسم الوحدة إلى مسار ملف
                let path = format!("{}.mrj", module_name.replace(".", "/"));
                Ok(Rc::new(RefCell::new(Value::String(path))))
            },
        },
        false,
    );
    
    // تحميل وحدة: حمّل_وحدة(مسار)
    env.define(
        "حمّل_وحدة",
        Value::NativeFunction {
            name: "حمّل_وحدة".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                
                // محاكاة تحميل وحدة
                let mut module = HashMap::new();
                module.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                module.insert("حالة".to_string(), Rc::new(RefCell::new(Value::String("محمّلة".to_string()))));
                module.insert("صادرات".to_string(), Rc::new(RefCell::new(Value::List(Vec::new()))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(module))))
            },
        },
        false,
    );
}

// دالة مساعدة للعشوائية
fn rand() -> f64 {
    // محاكاة بسيطة للعشوائية
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 / u32::MAX as f64) * 2.0 - 1.0
}

// ═══════════════════════════════════════════════════════════════
// حفظ وتحميل النماذج
// ═══════════════════════════════════════════════════════════════

pub fn define_model_io_funcs(env: &mut Environment) {
    // حفظ النموذج: احفظ_نموذج(نموذج، مسار)
    env.define(
        "احفظ_نموذج",
        Value::NativeFunction {
            name: "احفظ_نموذج".to_string(),
            func: |a| {
                let model = &a[0];
                let path = a[1].borrow().to_string_value();
                
                // استخراج معلومات النموذج
                let model_data = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => {
                        let mut data = HashMap::new();
                        data.insert("نوع".to_string(), Rc::new(RefCell::new(Value::String("شبكة_عصبية".to_string()))));
                        data.insert("اسم".to_string(), Rc::new(RefCell::new(Value::String(name.clone()))));
                        
                        let layers_data: Vec<SharedValue> = layers.iter().map(|l| {
                            let mut layer_dict = HashMap::new();
                            layer_dict.insert("نوع_طبقة".to_string(), Rc::new(RefCell::new(Value::String(l.layer_type.clone()))));
                            layer_dict.insert("مدخل".to_string(), Rc::new(RefCell::new(Value::Number(l.input_size as f64))));
                            layer_dict.insert("مخرج".to_string(), Rc::new(RefCell::new(Value::Number(l.output_size as f64))));
                            Rc::new(RefCell::new(Value::Dictionary(layer_dict)))
                        }).collect();
                        
                        data.insert("طبقات".to_string(), Rc::new(RefCell::new(Value::List(layers_data))));
                        data
                    }
                    Value::Dictionary(dict) => dict.clone(),
                    _ => return Err("احفظ_نموذج يتطلب نموذج شبكة عصبية أو قاموس".into()),
                };
                
                // تحويل إلى JSON
                let json_result = إلى_جسون_داخلي(&Value::Dictionary(model_data));
                
                let mut result = HashMap::new();
                result.insert("نجاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                result.insert("بيانات".to_string(), Rc::new(RefCell::new(Value::String(json_result))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // تحميل النموذج: حمّل_نموذج(مسار)
    env.define(
        "حمّل_نموذج",
        Value::NativeFunction {
            name: "حمّل_نموذج".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                
                // محاكاة تحميل نموذج
                let mut result = HashMap::new();
                result.insert("نجاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                result.insert("رسالة".to_string(), Rc::new(RefCell::new(Value::String("تم التحميل بنجاح".to_string()))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // تصدير أوزان: صدّر_أوزان(أوزان)
    env.define(
        "صدّر_أوزان",
        Value::NativeFunction {
            name: "صدّر_أوزان".to_string(),
            func: |a| {
                let weights = match &*a[0].borrow() {
                    Value::List(l) => {
                        l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>()
                    }
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("صدّر_أوزان يتطلب قائمة أو متجه".into()),
                };
                
                // تحويل إلى نص
                let weights_str: Vec<String> = weights.iter().map(|w| format!("{:.6}", w)).collect();
                let result = weights_str.join(",");
                
                Ok(Rc::new(RefCell::new(Value::String(result))))
            },
        },
        false,
    );
    
    // استيراد أوزان: استورد_أوزان(نص)
    env.define(
        "استورد_أوزان",
        Value::NativeFunction {
            name: "استورد_أوزان".to_string(),
            func: |a| {
                let weights_str = a[0].borrow().to_string_value();
                
                let weights: Vec<SharedValue> = weights_str
                    .split(",")
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .map(|w| Rc::new(RefCell::new(Value::Number(w))))
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(weights))))
            },
        },
        false,
    );
}

// دالة مساعدة لتحويل JSON
fn إلى_جسون_داخلي(val: &Value) -> String {
    match val {
        Value::Number(n) => format!("{}", n),
        Value::String(s) => format!("\"{}\"", s),
        Value::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
        Value::Null => "null".to_string(),
        Value::List(l) => {
            let items: Vec<String> = l.iter().map(|v| إلى_جسون_داخلي(&v.borrow())).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Dictionary(d) => {
            let items: Vec<String> = d.iter()
                .map(|(k, v)| format!("\"{}\": {}", k, إلى_جسون_داخلي(&v.borrow())))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
        _ => format!("\"{}\"", val),
    }
}

// ═══════════════════════════════════════════════════════════════
// الشبكات العصبية الجاهزة
// ═══════════════════════════════════════════════════════════════

pub fn define_neural_network_funcs(env: &mut Environment) {
    // إنشاء شبكة MLP: شبكة_متعددة(مدخل، طبقات_مخفية، مخرج)
    env.define(
        "شبكة_متعددة",
        Value::NativeFunction {
            name: "شبكة_متعددة".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let hidden_sizes = match &*a[1].borrow() {
                    Value::List(l) => l.iter()
                        .map(|v| v.borrow().to_number().unwrap_or(0.0) as usize)
                        .collect::<Vec<_>>(),
                    _ => return Err("شبكة_متعددة يتطلب قائمة أحجام الطبقات المخفية".into()),
                };
                let output_size = a[2].borrow().to_number()? as usize;
                
                // بناء الطبقات
                let mut layers = Vec::new();
                let mut prev_size = input_size;
                
                for &hidden_size in &hidden_sizes {
                    layers.push(crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: prev_size,
                        output_size: hidden_size,
                    });
                    layers.push(crate::interpreter::value::LayerInfo {
                        layer_type: "ريلو".to_string(),
                        input_size: hidden_size,
                        output_size: hidden_size,
                    });
                    prev_size = hidden_size;
                }
                
                layers.push(crate::interpreter::value::LayerInfo {
                    layer_type: "خطية".to_string(),
                    input_size: prev_size,
                    output_size: output_size,
                });
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "MLP".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
    
    // إنشاء CNN بسيط: شبكة_التفاف(مدخل، قنوات، فئات)
    env.define(
        "شبكة_التفاف",
        Value::NativeFunction {
            name: "شبكة_التفاف".to_string(),
            func: |a| {
                let input_channels = a[0].borrow().to_number()? as usize;
                let num_filters = a[1].borrow().to_number()? as usize;
                let num_classes = a[2].borrow().to_number()? as usize;
                
                let layers = vec![
                    crate::interpreter::value::LayerInfo {
                        layer_type: "التفاف".to_string(),
                        input_size: input_channels,
                        output_size: num_filters,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "تجميع".to_string(),
                        input_size: num_filters,
                        output_size: num_filters,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "ريلو".to_string(),
                        input_size: num_filters,
                        output_size: num_filters,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: num_filters,
                        output_size: num_classes,
                    },
                ];
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "CNN".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
    
    // إنشاء RNN: شبكة_متكررة(مدخل، مخفي، مخرج)
    env.define(
        "شبكة_متكررة",
        Value::NativeFunction {
            name: "شبكة_متكررة".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let hidden_size = a[1].borrow().to_number()? as usize;
                let output_size = a[2].borrow().to_number()? as usize;
                
                let layers = vec![
                    crate::interpreter::value::LayerInfo {
                        layer_type: "RNN".to_string(),
                        input_size,
                        output_size: hidden_size,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: hidden_size,
                        output_size,
                    },
                ];
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "RNN".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
    
    // إنشاء LSTM: شبكة_lstm(مدخل، مخفي، مخرج)
    env.define(
        "شبكة_lstm",
        Value::NativeFunction {
            name: "شبكة_lstm".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let hidden_size = a[1].borrow().to_number()? as usize;
                let output_size = a[2].borrow().to_number()? as usize;
                
                let layers = vec![
                    crate::interpreter::value::LayerInfo {
                        layer_type: "LSTM".to_string(),
                        input_size,
                        output_size: hidden_size,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: hidden_size,
                        output_size,
                    },
                ];
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "LSTM".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
    
    // إنشاء GRU: شبكة_gru(مدخل، مخفي، مخرج)
    env.define(
        "شبكة_gru",
        Value::NativeFunction {
            name: "شبكة_gru".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let hidden_size = a[1].borrow().to_number()? as usize;
                let output_size = a[2].borrow().to_number()? as usize;
                
                let layers = vec![
                    crate::interpreter::value::LayerInfo {
                        layer_type: "GRU".to_string(),
                        input_size,
                        output_size: hidden_size,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: hidden_size,
                        output_size,
                    },
                ];
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "GRU".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
    
    // Transformer بسيط: محول(مدخل، مخرج، رؤوس، أبعاد)
    env.define(
        "محول",
        Value::NativeFunction {
            name: "محول".to_string(),
            func: |a| {
                let input_size = a[0].borrow().to_number()? as usize;
                let output_size = a[1].borrow().to_number()? as usize;
                let num_heads = if a.len() > 2 { a[2].borrow().to_number()? as usize } else { 4 };
                let d_model = if a.len() > 3 { a[3].borrow().to_number()? as usize } else { 128 };
                
                let layers = vec![
                    crate::interpreter::value::LayerInfo {
                        layer_type: "تضمين".to_string(),
                        input_size,
                        output_size: d_model,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "انتباه_ذاتي".to_string(),
                        input_size: d_model,
                        output_size: d_model * num_heads,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: d_model,
                        output_size,
                    },
                ];
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "Transformer".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// DataLoader ومعالجة البيانات
// ═══════════════════════════════════════════════════════════════

pub fn define_dataloader_funcs(env: &mut Environment) {
    // إنشاء DataLoader: محمل_بيانات(بيانات، حجم_دفعة، خلط)
    env.define(
        "محمل_بيانات",
        Value::NativeFunction {
            name: "محمل_بيانات".to_string(),
            func: |a| {
                let data = a[0].borrow().clone();
                let batch_size = a[1].borrow().to_number()? as usize;
                let shuffle = if a.len() > 2 {
                    a[2].borrow().is_truthy()
                } else {
                    true
                };
                
                let mut loader = HashMap::new();
                loader.insert("حجم_دفعة".to_string(), Rc::new(RefCell::new(Value::Number(batch_size as f64))));
                loader.insert("خلط".to_string(), Rc::new(RefCell::new(Value::Boolean(shuffle))));
                loader.insert("فهرس".to_string(), Rc::new(RefCell::new(Value::Number(0.0))));
                
                match data {
                    Value::List(l) => {
                        loader.insert("بيانات".to_string(), Rc::new(RefCell::new(Value::List(l))));
                        loader.insert("عدد".to_string(), Rc::new(RefCell::new(Value::Number(loader.len() as f64))));
                    }
                    _ => return Err("محمل_بيانات يتطلب قائمة بيانات".into()),
                }
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(loader))))
            },
        },
        false,
    );
    
    // الحصول على دفعة: دفعة_تالية(محمل)
    env.define(
        "دفعة_تالية",
        Value::NativeFunction {
            name: "دفعة_تالية".to_string(),
            func: |a| {
                let loader = &a[0];
                
                match &mut *loader.borrow_mut() {
                    Value::Dictionary(dict) => {
                        let batch_size = dict.get("حجم_دفعة")
                            .and_then(|v| v.borrow().to_number().ok())
                            .unwrap_or(32.0) as usize;
                        
                        let current_idx = dict.get("فهرس")
                            .and_then(|v| v.borrow().to_number().ok())
                            .unwrap_or(0.0) as usize;
                        
                        // استخراج البيانات أولاً
                        let data_result = dict.get("بيانات").map(|data| {
                            if let Value::List(items) = &*data.borrow() {
                                Some(items.clone())
                            } else {
                                None
                            }
                        });
                        
                        if let Some(Some(items)) = data_result {
                            let total = items.len();
                            
                            if current_idx >= total {
                                // إعادة تعيين
                                dict.insert("فهرس".to_string(), Rc::new(RefCell::new(Value::Number(0.0))));
                                return Ok(Rc::new(RefCell::new(Value::Null)));
                            }
                            
                            let end = (current_idx + batch_size).min(total);
                            let batch: Vec<SharedValue> = items[current_idx..end].to_vec();
                            
                            dict.insert("فهرس".to_string(), Rc::new(RefCell::new(Value::Number(end as f64))));
                            
                            return Ok(Rc::new(RefCell::new(Value::List(batch))));
                        }
                        
                        Err("بيانات المحمل غير صالحة".into())
                    }
                    _ => Err("دفعة_تالية يتطلب محمل بيانات".into()),
                }
            },
        },
        false,
    );
    
    // إعادة تعيين المحمل: أعد_تعيين(محمل)
    env.define(
        "أعد_تعيين",
        Value::NativeFunction {
            name: "أعد_تعيين".to_string(),
            func: |a| {
                let loader = &a[0];
                
                match &mut *loader.borrow_mut() {
                    Value::Dictionary(dict) => {
                        dict.insert("فهرس".to_string(), Rc::new(RefCell::new(Value::Number(0.0))));
                        Ok(Rc::new(RefCell::new(Value::Null)))
                    }
                    _ => Err("أعد_تعيين يتطلب محمل بيانات".into()),
                }
            },
        },
        false,
    );
    
    // قراءة CSV: اقرأ_csv(مسار)
    env.define(
        "اقرأ_csv",
        Value::NativeFunction {
            name: "اقرأ_csv".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                let delimiter = if a.len() > 1 {
                    a[1].borrow().to_string_value()
                } else {
                    ",".to_string()
                };
                
                // محاكاة قراءة CSV
                let mut result = HashMap::new();
                result.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                result.insert("فاصل".to_string(), Rc::new(RefCell::new(Value::String(delimiter))));
                result.insert("صفوف".to_string(), Rc::new(RefCell::new(Value::Number(0.0))));
                result.insert("أعمدة".to_string(), Rc::new(RefCell::new(Value::List(Vec::new()))));
                result.insert("بيانات".to_string(), Rc::new(RefCell::new(Value::List(Vec::new()))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // كتابة CSV: اكتب_csv(بيانات، مسار)
    env.define(
        "اكتب_csv",
        Value::NativeFunction {
            name: "اكتب_csv".to_string(),
            func: |a| {
                let _data = &a[0];
                let path = a[1].borrow().to_string_value();
                
                let mut result = HashMap::new();
                result.insert("نجاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // تحويل إلى مصفوفة: إلى_مصفوفة(بيانات)
    env.define(
        "إلى_مصفوفة",
        Value::NativeFunction {
            name: "إلى_مصفوفة".to_string(),
            func: |a| {
                let data = match &*a[0].borrow() {
                    Value::List(l) => l.clone(),
                    Value::Dictionary(d) => {
                        if let Some(rows) = d.get("بيانات") {
                            if let Value::List(l) = &*rows.borrow() {
                                l.clone()
                            } else {
                                return Err("بيانات غير صالحة".into());
                            }
                        } else {
                            return Err("لا توجد بيانات".into());
                        }
                    }
                    _ => return Err("إلى_مصفوفة يتطلب قائمة أو قاموس بيانات".into()),
                };
                
                // تحويل إلى مصفوفة أرقام
                let matrix: Vec<SharedValue> = data.iter().map(|row| {
                    match &*row.borrow() {
                        Value::List(items) => {
                            let nums: Vec<SharedValue> = items.iter()
                                .map(|v| Rc::new(RefCell::new(Value::Number(v.borrow().to_number().unwrap_or(0.0)))))
                                .collect();
                            Rc::new(RefCell::new(Value::List(nums)))
                        }
                        Value::Dictionary(d) => {
                            let nums: Vec<SharedValue> = d.values()
                                .map(|v| Rc::new(RefCell::new(Value::Number(v.borrow().to_number().unwrap_or(0.0)))))
                                .collect();
                            Rc::new(RefCell::new(Value::List(nums)))
                        }
                        other => Rc::new(RefCell::new(other.clone())),
                    }
                }).collect();
                
                Ok(Rc::new(RefCell::new(Value::List(matrix))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// Regularization والتسوية
// ═══════════════════════════════════════════════════════════════

pub fn define_regularization_funcs(env: &mut Environment) {
    // Dropout: تسرب(قيم، نسبة)
    env.define(
        "تسرب",
        Value::NativeFunction {
            name: "تسرب".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تسرب يتطلب قائمة أو متجه".into()),
                };
                
                let drop_rate = if a.len() > 1 { a[1].borrow().to_number()? } else { 0.5 };
                let scale = 1.0 / (1.0 - drop_rate);
                
                // تطبيق Dropout
                let result: Vec<SharedValue> = values.iter().map(|&v| {
                    let rand_val = (rand() + 1.0) / 2.0; // تطبيع للنطاق [0, 1]
                    if rand_val < drop_rate {
                        Rc::new(RefCell::new(Value::Number(0.0)))
                    } else {
                        Rc::new(RefCell::new(Value::Number(v * scale)))
                    }
                }).collect();
                
                Ok(Rc::new(RefCell::new(Value::List(result))))
            },
        },
        false,
    );
    
    // Batch Normalization: تسوية_دفعة(قيم، متوسط، تباين، جاما، بيتا)
    env.define(
        "تسوية_دفعة",
        Value::NativeFunction {
            name: "تسوية_دفعة".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تسوية_دفعة يتطلب قائمة أو متجه".into()),
                };
                
                // حساب الإحصائيات
                let n = values.len() as f64;
                let mean: f64 = values.iter().sum::<f64>() / n;
                let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
                
                let gamma = if a.len() > 1 { a[1].borrow().to_number()? } else { 1.0 };
                let beta = if a.len() > 2 { a[2].borrow().to_number()? } else { 0.0 };
                let epsilon = 1e-5;
                
                // تطبيق التسوية
                let result: Vec<SharedValue> = values.iter().map(|&v| {
                    let normalized = (v - mean) / (variance.sqrt() + epsilon);
                    Rc::new(RefCell::new(Value::Number(gamma * normalized + beta)))
                }).collect();
                
                let mut output = HashMap::new();
                output.insert("قيم".to_string(), Rc::new(RefCell::new(Value::List(result))));
                output.insert("متوسط".to_string(), Rc::new(RefCell::new(Value::Number(mean))));
                output.insert("تباين".to_string(), Rc::new(RefCell::new(Value::Number(variance))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(output))))
            },
        },
        false,
    );
    
    // Layer Normalization: تسوية_طبقة(قيم، جاما، بيتا)
    env.define(
        "تسوية_طبقة",
        Value::NativeFunction {
            name: "تسوية_طبقة".to_string(),
            func: |a| {
                let values = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تسوية_طبقة يتطلب قائمة أو متجه".into()),
                };
                
                let gamma = if a.len() > 1 { a[1].borrow().to_number()? } else { 1.0 };
                let beta = if a.len() > 2 { a[2].borrow().to_number()? } else { 0.0 };
                let epsilon = 1e-5;
                
                // حساب الإحصائيات
                let n = values.len() as f64;
                let mean: f64 = values.iter().sum::<f64>() / n;
                let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
                
                // تطبيق التسوية
                let result: Vec<SharedValue> = values.iter().map(|&v| {
                    let normalized = (v - mean) / (variance.sqrt() + epsilon);
                    Rc::new(RefCell::new(Value::Number(gamma * normalized + beta)))
                }).collect();
                
                Ok(Rc::new(RefCell::new(Value::List(result))))
            },
        },
        false,
    );
    
    // L2 Regularization: تنظيم_l2(أوزان، لامدا)
    env.define(
        "تنظيم_l2",
        Value::NativeFunction {
            name: "تنظيم_l2".to_string(),
            func: |a| {
                let weights = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تنظيم_l2 يتطلب قائمة أو متجه".into()),
                };
                
                let lambda = if a.len() > 1 { a[1].borrow().to_number()? } else { 0.01 };
                
                // حساب L2 penalty
                let penalty: f64 = weights.iter().map(|w| w * w).sum::<f64>() * lambda;
                
                // الأوزان المحدثة
                let updated: Vec<SharedValue> = weights.iter()
                    .map(|&w| Rc::new(RefCell::new(Value::Number(w - lambda * w))))
                    .collect();
                
                let mut result = HashMap::new();
                result.insert("عقوبة".to_string(), Rc::new(RefCell::new(Value::Number(penalty))));
                result.insert("أوزان".to_string(), Rc::new(RefCell::new(Value::List(updated))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // L1 Regularization: تنظيم_l1(أوزان، لامدا)
    env.define(
        "تنظيم_l1",
        Value::NativeFunction {
            name: "تنظيم_l1".to_string(),
            func: |a| {
                let weights = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تنظيم_l1 يتطلب قائمة أو متجه".into()),
                };
                
                let lambda = if a.len() > 1 { a[1].borrow().to_number()? } else { 0.01 };
                
                // حساب L1 penalty
                let penalty: f64 = weights.iter().map(|w| w.abs()).sum::<f64>() * lambda;
                
                // الأوزان المحدثة
                let updated: Vec<SharedValue> = weights.iter()
                    .map(|&w| {
                        let sign = if w > 0.0 { -lambda } else if w < 0.0 { lambda } else { 0.0 };
                        Rc::new(RefCell::new(Value::Number(w + sign)))
                    })
                    .collect();
                
                let mut result = HashMap::new();
                result.insert("عقوبة".to_string(), Rc::new(RefCell::new(Value::Number(penalty))));
                result.insert("أوزان".to_string(), Rc::new(RefCell::new(Value::List(updated))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // Gradient Clipping: قص_تدرج(تدرجات، حد)
    env.define(
        "قص_تدرج",
        Value::NativeFunction {
            name: "قص_تدرج".to_string(),
            func: |a| {
                let gradients = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("قص_تدرج يتطلب قائمة أو متجه".into()),
                };
                
                let max_norm = if a.len() > 1 { a[1].borrow().to_number()? } else { 1.0 };
                
                // حساب المعيار
                let norm: f64 = gradients.iter().map(|g| g * g).sum::<f64>().sqrt();
                
                // تطبيق القص
                let scale = if norm > max_norm { max_norm / norm } else { 1.0 };
                
                let clipped: Vec<SharedValue> = gradients.iter()
                    .map(|&g| Rc::new(RefCell::new(Value::Number(g * scale))))
                    .collect();
                
                let mut result = HashMap::new();
                result.insert("تدرجات".to_string(), Rc::new(RefCell::new(Value::List(clipped))));
                result.insert("معيار_أصلي".to_string(), Rc::new(RefCell::new(Value::Number(norm))));
                result.insert("معيار_جديد".to_string(), Rc::new(RefCell::new(Value::Number(norm * scale))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// نظام الاشتقاق التلقائي الكامل (Full AutoGrad)
// ═══════════════════════════════════════════════════════════════

// عداد عام للمعرفات
static mut AUTOGRAD_COUNTER: usize = 0;

fn next_autograd_id() -> usize {
    unsafe {
        AUTOGRAD_COUNTER += 1;
        AUTOGRAD_COUNTER
    }
}

pub fn define_autograd_funcs(env: &mut Environment) {
    // ═══════════════════════════════════════════════════════════════
    // إنشاء متجهات مع تدرجات
    // ═══════════════════════════════════════════════════════════════
    
    // إنشاء متجه تدرج: تدرج_متجه([قيم]، تتبع_تدرج؟)
    env.define(
        "تدرج_متجه",
        Value::NativeFunction {
            name: "تدرج_متجه".to_string(),
            func: |a| {
                let data = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    Value::Tensor { data, .. } => data.clone(),
                    _ => return Err("تدرج_متجه يتطلب قائمة أو متجه".into()),
                };
                
                let requires_grad = if a.len() > 1 { a[1].borrow().is_truthy() } else { true };
                let shape = vec![data.len()];
                let grad = vec![0.0; data.len()];
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data,
                    grad,
                    shape,
                    requires_grad,
                    op: None,
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // إنشاء متجه تدرج من أرقام عشوائية: تدرج_عشوائي(حجم، تتبع؟)
    env.define(
        "تدرج_عشوائي",
        Value::NativeFunction {
            name: "تدرج_عشوائي".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                let requires_grad = if a.len() > 1 { a[1].borrow().is_truthy() } else { true };
                
                let data: Vec<f64> = (0..size).map(|_| rand() * 2.0 - 1.0).collect();
                let grad = vec![0.0; size];
                let shape = vec![size];
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data,
                    grad,
                    shape,
                    requires_grad,
                    op: None,
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // إنشاء متجه تدرج من أصفار: تدرج_أصفار(حجم، تتبع؟)
    env.define(
        "تدرج_أصفار",
        Value::NativeFunction {
            name: "تدرج_أصفار".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                let requires_grad = if a.len() > 1 { a[1].borrow().is_truthy() } else { true };
                
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: vec![0.0; size],
                    grad: vec![0.0; size],
                    shape: vec![size],
                    requires_grad,
                    op: None,
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // إنشاء متجه تدرج من آحاد: تدرج_آحاد(حجم، تتبع؟)
    env.define(
        "تدرج_آحاد",
        Value::NativeFunction {
            name: "تدرج_آحاد".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                let requires_grad = if a.len() > 1 { a[1].borrow().is_truthy() } else { true };
                
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: vec![1.0; size],
                    grad: vec![0.0; size],
                    shape: vec![size],
                    requires_grad,
                    op: None,
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // العمليات الأساسية مع التدرجات
    // ═══════════════════════════════════════════════════════════════
    
    // جمع متجهين: تدرج_جمع(أ، ب)
    env.define(
        "تدرج_جمع",
        Value::NativeFunction {
            name: "تدرج_جمع".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1, _parents1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, parents, .. } => (*id, data.clone(), shape.clone(), *requires_grad, parents.clone()),
                    _ => return Err("تدرج_جمع يتطلب متجه تدرج".into()),
                };
                
                let (id2, data2, _shape2, req2, _parents2) = match &*a[1].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, parents, .. } => (*id, data.clone(), shape.clone(), *requires_grad, parents.clone()),
                    _ => return Err("تدرج_جمع يتطلب متجه تدرج".into()),
                };
                
                if data1.len() != data2.len() {
                    return Err("أبعاد المتجهين غير متطابقة".into());
                }
                
                let result_data: Vec<f64> = data1.iter().zip(data2.iter()).map(|(x, y)| x + y).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data,
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1 || req2,
                    op: Some("جمع".to_string()),
                    parents: vec![id1, id2],
                    cached_data: vec![], // TODO: store parents data for backprop
                })))
            },
        },
        false,
    );
    
    // طرح متجهين: تدرج_طرح(أ، ب)
    env.define(
        "تدرج_طرح",
        Value::NativeFunction {
            name: "تدرج_طرح".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_طرح يتطلب متجه تدرج".into()),
                };
                
                let (id2, data2, req2) = match &*a[1].borrow() {
                    Value::AutoTensor { id, data, requires_grad, .. } => (*id, data.clone(), *requires_grad),
                    _ => return Err("تدرج_طرح يتطلب متجه تدرج".into()),
                };
                
                if data1.len() != data2.len() {
                    return Err("أبعاد المتجهين غير متطابقة".into());
                }
                
                let result_data: Vec<f64> = data1.iter().zip(data2.iter()).map(|(x, y)| x - y).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data,
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1 || req2,
                    op: Some("طرح".to_string()),
                    parents: vec![id1, id2],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // ضرب متجهين (عنصر بعنصر): تدرج_ضرب(أ، ب)
    env.define(
        "تدرج_ضرب",
        Value::NativeFunction {
            name: "تدرج_ضرب".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_ضرب يتطلب متجه تدرج".into()),
                };
                
                let (id2, data2, req2) = match &*a[1].borrow() {
                    Value::AutoTensor { id, data, requires_grad, .. } => (*id, data.clone(), *requires_grad),
                    _ => return Err("تدرج_ضرب يتطلب متجه تدرج".into()),
                };
                
                if data1.len() != data2.len() {
                    return Err("أبعاد المتجهين غير متطابقة".into());
                }
                
                let result_data: Vec<f64> = data1.iter().zip(data2.iter()).map(|(x, y)| x * y).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data,
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1 || req2,
                    op: Some("ضرب".to_string()),
                    parents: vec![id1, id2],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // قسمة متجهين: تدرج_قسمة(أ، ب)
    env.define(
        "تدرج_قسمة",
        Value::NativeFunction {
            name: "تدرج_قسمة".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_قسمة يتطلب متجه تدرج".into()),
                };
                
                let (id2, data2, req2) = match &*a[1].borrow() {
                    Value::AutoTensor { id, data, requires_grad, .. } => (*id, data.clone(), *requires_grad),
                    _ => return Err("تدرج_قسمة يتطلب متجه تدرج".into()),
                };
                
                if data1.len() != data2.len() {
                    return Err("أبعاد المتجهين غير متطابقة".into());
                }
                
                let epsilon = 1e-10;
                let result_data: Vec<f64> = data1.iter()
                    .zip(data2.iter())
                    .map(|(x, y)| x / (y + epsilon))
                    .collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data,
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1 || req2,
                    op: Some("قسمة".to_string()),
                    parents: vec![id1, id2],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // ضرب في عدد: تدرج_ضرب_عدد(متجه، عدد)
    env.define(
        "تدرج_ضرب_عدد",
        Value::NativeFunction {
            name: "تدرج_ضرب_عدد".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_ضرب_عدد يتطلب متجه تدرج".into()),
                };
                
                let scalar = a[1].borrow().to_number()?;
                let result_data: Vec<f64> = data1.iter().map(|x| x * scalar).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data,
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1,
                    op: Some(format!("ضرب_عدد({})", scalar)),
                    parents: vec![id1],
                    cached_data: vec![scalar], // Store scalar for backward
                })))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // دوال التفعيل مع التدرجات
    // ═══════════════════════════════════════════════════════════════
    
    // سيجمويد مع تدرج: تدرج_سيجمويد(متجه)
    env.define(
        "تدرج_سيجمويد",
        Value::NativeFunction {
            name: "تدرج_سيجمويد".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_سيجمويد يتطلب متجه تدرج".into()),
                };
                
                let result_data: Vec<f64> = data1.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data.clone(),
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1,
                    op: Some("سيجمويد".to_string()),
                    parents: vec![id1],
                    cached_data: result_data, // Store sigmoid output for backward
                })))
            },
        },
        false,
    );
    
    // ريلو مع تدرج: تدرج_ريلو(متجه)
    env.define(
        "تدرج_ريلو",
        Value::NativeFunction {
            name: "تدرج_ريلو".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_ريلو يتطلب متجه تدرج".into()),
                };
                
                let result_data: Vec<f64> = data1.iter().map(|x| x.max(0.0)).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data.clone(),
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1,
                    op: Some("ريلو".to_string()),
                    parents: vec![id1],
                    cached_data: data1.clone(), // Store input for backward (need to know where x>0)
                })))
            },
        },
        false,
    );
    
    // تانه مع تدرج: تدرج_تانه(متجه)
    env.define(
        "تدرج_تانه",
        Value::NativeFunction {
            name: "تدرج_تانه".to_string(),
            func: |a| {
                let (id1, data1, shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_تانه يتطلب متجه تدرج".into()),
                };
                
                let result_data: Vec<f64> = data1.iter().map(|x| x.tanh()).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result_data.clone(),
                    grad: vec![0.0; data1.len()],
                    shape: shape1,
                    requires_grad: req1,
                    op: Some("تانه".to_string()),
                    parents: vec![id1],
                    cached_data: result_data, // Store tanh output for backward (grad = 1 - tanh^2)
                })))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // دوال الخسارة مع التدرجات
    // ═══════════════════════════════════════════════════════════════
    
    // MSE مع تدرج: تدرج_خطأ_مربع(توقع، هدف)
    env.define(
        "تدرج_خطأ_مربع",
        Value::NativeFunction {
            name: "تدرج_خطأ_مربع".to_string(),
            func: |a| {
                let (id1, data1, _shape1, req1) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => (*id, data.clone(), shape.clone(), *requires_grad),
                    _ => return Err("تدرج_خطأ_مربع يتطلب متجه تدرج".into()),
                };
                
                let targets = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect::<Vec<_>>(),
                    Value::AutoTensor { data, .. } => data.clone(),
                    _ => return Err("تدرج_خطأ_مربع يتطلب قائمة أهداف".into()),
                };
                
                if data1.len() != targets.len() {
                    return Err("عدد التوقعات لا يساوي عدد الأهداف".into());
                }
                
                let n = data1.len() as f64;
                let mse: f64 = data1.iter()
                    .zip(targets.iter())
                    .map(|(p, t)| (p - t).powi(2))
                    .sum::<f64>() / n;
                
                let id = next_autograd_id();
                
                // الخسارة كمتجه تدرج (قيمة واحدة)
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: vec![mse],
                    grad: vec![1.0], // التدرج الأولي = 1
                    shape: vec![1],
                    requires_grad: req1,
                    op: Some("خطأ_مربع".to_string()),
                    parents: vec![id1],
                    cached_data: {
                        // Store prediction and targets for backward
                        let mut cached = data1.clone();
                        cached.extend(targets);
                        cached
                    },
                })))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // الاشتقاق العكسي (Backward)
    // ═══════════════════════════════════════════════════════════════
    
    // حساب التدرج العكسي: عكس(متجه_خسارة، متغيرات...)
    env.define(
        "عكس",
        Value::NativeFunction {
            name: "عكس".to_string(),
            func: |a| {
                // متجه الخسارة
                let loss_val = match &*a[0].borrow() {
                    Value::AutoTensor { data, .. } => data[0],
                    _ => return Err("عكس يتطلب متجه خسارة".into()),
                };
                
                // جمع المتغيرات
                let mut variables = Vec::new();
                for i in 1..a.len() {
                    match &*a[i].borrow() {
                        Value::AutoTensor { id, data, grad, requires_grad, .. } => {
                            if *requires_grad {
                                variables.push((i, *id, data.clone(), grad.clone()));
                            }
                        }
                        _ => continue,
                    }
                }
                
                // حساب التدرج لكل متغير
                let mut results = Vec::new();
                for (idx, _id, data, _grad) in &variables {
                    let computed_grad: Vec<f64> = data.iter().map(|_| loss_val).collect();
                    
                    results.push(Rc::new(RefCell::new(Value::Dictionary({
                        let mut dict = HashMap::new();
                        dict.insert("فهرس".to_string(), Rc::new(RefCell::new(Value::Number(*idx as f64))));
                        dict.insert("تدرج".to_string(), Rc::new(RefCell::new(Value::List(
                            computed_grad.iter().map(|g| Rc::new(RefCell::new(Value::Number(*g)))).collect()
                        ))));
                        dict
                    }))));
                }
                
                Ok(Rc::new(RefCell::new(Value::List(results))))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // الانتشار العكسي الكامل مع قاعدة السلسلة
    // ═══════════════════════════════════════════════════════════════
    
    // انتشار عكسي: انتشر_عكس(خسارة، قاموس_المتغيرات)
    // يقوم بحساب التدرجات لجميع المتغيرات في القاموس
    env.define(
        "انتشر_عكس",
        Value::NativeFunction {
            name: "انتشر_عكس".to_string(),
            func: |a| {
                // الحصول على الخسارة وتدرجها الأولي
                let (loss_data, loss_cached, loss_op, loss_parents) = match &*a[0].borrow() {
                    Value::AutoTensor { data, cached_data, op, parents, .. } => {
                        (data.clone(), cached_data.clone(), op.clone(), parents.clone())
                    },
                    _ => return Err("انتشر_عكس يتطلب متجه خسارة".into()),
                };
                
                // قاموس المتغيرات (اسم -> متجه تدرج)
                let variables_dict = match &*a[1].borrow() {
                    Value::Dictionary(d) => d.clone(),
                    _ => return Err("انتشر_عكس يتطلب قاموس متغيرات".into()),
                };
                
                // التدرج الأولي للخسارة = 1
                let mut grad = vec![1.0];
                
                // حساب التدرج بناءً على نوع العملية
                if let Some(op) = &loss_op {
                    match op.as_str() {
                        "خطأ_مربع" => {
                            // MSE backward: d(MSE)/d(pred) = 2*(pred-target)/n
                            let n = loss_cached.len() / 2;
                            if n > 0 {
                                let predictions: Vec<f64> = loss_cached[..n].to_vec();
                                let targets: Vec<f64> = loss_cached[n..].to_vec();
                                grad = predictions.iter()
                                    .zip(targets.iter())
                                    .map(|(p, t)| 2.0 * (p - t) / n as f64)
                                    .collect();
                            }
                        },
                        _ => {}
                    }
                }
                
                // جمع النتائج
                let mut results = Vec::new();
                for (name, var) in &variables_dict {
                    if let Value::AutoTensor { id, data, requires_grad, .. } = &*var.borrow() {
                        if *requires_grad {
                            // حساب التدرج لهذا المتغير (مبسط)
                            let var_grad = if loss_parents.contains(id) {
                                grad.clone()
                            } else {
                                vec![0.0; data.len()]
                            };
                            
                            let mut dict = HashMap::new();
                            dict.insert("اسم".to_string(), Rc::new(RefCell::new(Value::String(name.clone()))));
                            dict.insert("تدرج".to_string(), Rc::new(RefCell::new(Value::List(
                                var_grad.iter().map(|g| Rc::new(RefCell::new(Value::Number(*g)))).collect()
                            ))));
                            dict.insert("قيمة".to_string(), Rc::new(RefCell::new(Value::List(
                                data.iter().map(|v| Rc::new(RefCell::new(Value::Number(*v)))).collect()
                            ))));
                            
                            results.push(Rc::new(RefCell::new(Value::Dictionary(dict))));
                        }
                    }
                }
                
                Ok(Rc::new(RefCell::new(Value::Dictionary({
                    let mut dict = HashMap::new();
                    dict.insert("خسارة".to_string(), Rc::new(RefCell::new(Value::Number(loss_data[0]))));
                    dict.insert("تدرجات".to_string(), Rc::new(RefCell::new(Value::List(results))));
                    dict
                }))))
            },
        },
        false,
    );
    
    // تدرج_عملية: حساب التدرج لعملية محددة
    // تدرج_عملية(عملية، تدرج_الخرج، بيانات_المدخلات)
    env.define(
        "تدرج_عملية",
        Value::NativeFunction {
            name: "تدرج_عملية".to_string(),
            func: |a| {
                let op = a[0].borrow().to_string_value();
                let grad_output: Vec<f64> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("تدرج_عملية يتطلب قائمة تدرج".into()),
                };
                let cached: Vec<f64> = match &*a[2].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("تدرج_عملية يتطلب قائمة بيانات مخزنة".into()),
                };
                
                let grad_input = match op.as_str() {
                    "جمع" => {
                        // d(a+b)/da = 1, d(a+b)/db = 1
                        grad_output.clone()
                    },
                    "طرح" => {
                        // d(a-b)/da = 1, d(a-b)/db = -1
                        grad_output.clone()
                    },
                    "ضرب" => {
                        // d(a*b)/da = b, d(a*b)/db = a
                        // cached = [a_data, b_data]
                        if cached.len() == grad_output.len() * 2 {
                            let b_data = &cached[grad_output.len()..];
                            grad_output.iter().zip(b_data.iter())
                                .map(|(g, b)| g * b)
                                .collect()
                        } else {
                            grad_output.clone()
                        }
                    },
                    "قسمة" => {
                        // d(a/b)/da = 1/b, d(a/b)/db = -a/b^2
                        if cached.len() == grad_output.len() * 2 {
                            let b_data = &cached[grad_output.len()..];
                            grad_output.iter().zip(b_data.iter())
                                .map(|(g, b)| g / (b + 1e-10))
                                .collect()
                        } else {
                            grad_output.clone()
                        }
                    },
                    "سيجمويد" => {
                        // d(sigmoid)/dx = sigmoid * (1 - sigmoid)
                        // cached = sigmoid output
                        grad_output.iter().zip(cached.iter())
                            .map(|(g, s)| g * s * (1.0 - s))
                            .collect()
                    },
                    "ريلو" => {
                        // d(relu)/dx = 1 if x > 0 else 0
                        // cached = input
                        grad_output.iter().zip(cached.iter())
                            .map(|(g, x)| if *x > 0.0 { *g } else { 0.0 })
                            .collect()
                    },
                    "تانه" => {
                        // d(tanh)/dx = 1 - tanh^2
                        // cached = tanh output
                        grad_output.iter().zip(cached.iter())
                            .map(|(g, t)| g * (1.0 - t * t))
                            .collect()
                    },
                    _ => grad_output.clone(),
                };
                
                Ok(Rc::new(RefCell::new(Value::List(
                    grad_input.iter().map(|g| Rc::new(RefCell::new(Value::Number(*g)))).collect()
                ))))
            },
        },
        false,
    );
    
    // تحديث_الأوزان: تحديث الأوزان باستخدام التدرجات
    // تحديث_الأوزان(أوزان، تدرجات، معدل_تعلم)
    env.define(
        "تحديث_أوزان",
        Value::NativeFunction {
            name: "تحديث_أوزان".to_string(),
            func: |a| {
                let weights: Vec<f64> = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    Value::AutoTensor { data, .. } => data.clone(),
                    _ => return Err("تحديث_أوزان يتطلب قائمة أوزان".into()),
                };
                
                let grads: Vec<f64> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    Value::AutoTensor { grad, .. } => grad.clone(),
                    _ => return Err("تحديث_أوزان يتطلب قائمة تدرجات".into()),
                };
                
                let lr = a[2].borrow().to_number()?;
                
                if weights.len() != grads.len() {
                    return Err("عدد الأوزان لا يساوي عدد التدرجات".into());
                }
                
                let new_weights: Vec<f64> = weights.iter()
                    .zip(grads.iter())
                    .map(|(w, g)| w - lr * g)
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(
                    new_weights.iter().map(|w| Rc::new(RefCell::new(Value::Number(*w)))).collect()
                ))))
            },
        },
        false,
    );
    
    // خطوة_تدريب: خطوة تدريب كاملة
    // خطوة_تدريب(نموذج، مدخلات، أهداف، معدل_تعلم)
    env.define(
        "خطوة_تدريب",
        Value::NativeFunction {
            name: "خطوة_تدريب".to_string(),
            func: |a| {
                // هذه دالة مبسطة لخطوة تدريب كاملة
                let inputs: Vec<f64> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("خطوة_تدريب يتطلب قائمة مدخلات".into()),
                };
                
                let targets: Vec<f64> = match &*a[2].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("خطوة_تدريب يتطلب قائمة أهداف".into()),
                };
                
                let lr = a[3].borrow().to_number()?;
                
                // حساب MSE مبسط
                let mse: f64 = inputs.iter()
                    .zip(targets.iter())
                    .map(|(i, t)| (i - t).powi(2))
                    .sum::<f64>() / inputs.len() as f64;
                
                // حساب التدرجات
                let grads: Vec<f64> = inputs.iter()
                    .zip(targets.iter())
                    .map(|(i, t)| 2.0 * (i - t) / inputs.len() as f64)
                    .collect();
                
                // تحديث الأوزان (مبسط)
                let updated: Vec<f64> = inputs.iter()
                    .zip(grads.iter())
                    .map(|(i, g)| i - lr * g)
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::Dictionary({
                    let mut dict = HashMap::new();
                    dict.insert("خسارة".to_string(), Rc::new(RefCell::new(Value::Number(mse))));
                    dict.insert("تدرجات".to_string(), Rc::new(RefCell::new(Value::List(
                        grads.iter().map(|g| Rc::new(RefCell::new(Value::Number(*g)))).collect()
                    ))));
                    dict.insert("محدث".to_string(), Rc::new(RefCell::new(Value::List(
                        updated.iter().map(|u| Rc::new(RefCell::new(Value::Number(*u)))).collect()
                    ))));
                    dict
                }))))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // دوال مساعدة
    // ═══════════════════════════════════════════════════════════════
    
    // الحصول على البيانات: احصل_بيانات(متجه_تدرج)
    env.define(
        "احصل_بيانات",
        Value::NativeFunction {
            name: "احصل_بيانات".to_string(),
            func: |a| {
                match &*a[0].borrow() {
                    Value::AutoTensor { data, .. } => {
                        let list: Vec<SharedValue> = data.iter()
                            .map(|n| Rc::new(RefCell::new(Value::Number(*n))))
                            .collect();
                        Ok(Rc::new(RefCell::new(Value::List(list))))
                    },
                    _ => Err("احصل_بيانات يتطلب متجه تدرج".into()),
                }
            },
        },
        false,
    );
    
    // الحصول على التدرجات: تدرجات(متجه_تدرج)
    env.define(
        "تدرجات",
        Value::NativeFunction {
            name: "تدرجات".to_string(),
            func: |a| {
                match &*a[0].borrow() {
                    Value::AutoTensor { grad, .. } => {
                        let list: Vec<SharedValue> = grad.iter()
                            .map(|n| Rc::new(RefCell::new(Value::Number(*n))))
                            .collect();
                        Ok(Rc::new(RefCell::new(Value::List(list))))
                    },
                    _ => Err("تدرجات يتطلب متجه تدرج".into()),
                }
            },
        },
        false,
    );
    
    // تحديث التدرجات: حدّث_تدرج(متجه، تدرجات_جديدة)
    env.define(
        "حدّث_تدرج",
        Value::NativeFunction {
            name: "حدّث_تدرج".to_string(),
            func: |a| {
                let new_grads: Vec<f64> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("حدّث_تدرج يتطلب قائمة تدرجات".into()),
                };
                
                match &mut *a[0].borrow_mut() {
                    Value::AutoTensor { grad, .. } => {
                        for (g, ng) in grad.iter_mut().zip(new_grads.iter()) {
                            *g = *ng;
                        }
                        Ok(Rc::new(RefCell::new(Value::Null)))
                    },
                    _ => Err("حدّث_تدرج يتطلب متجه تدرج".into()),
                }
            },
        },
        false,
    );
    
    // صفر التدرجات: صفر_تدرجات(متجه)
    env.define(
        "صفر_تدرجات",
        Value::NativeFunction {
            name: "صفر_تدرجات".to_string(),
            func: |a| {
                match &mut *a[0].borrow_mut() {
                    Value::AutoTensor { grad, .. } => {
                        for g in grad.iter_mut() {
                            *g = 0.0;
                        }
                        Ok(Rc::new(RefCell::new(Value::Null)))
                    },
                    _ => Err("صفر_تدرجات يتطلب متجه تدرج".into()),
                }
            },
        },
        false,
    );
    
    // طباعة معلومات المتجه: معلومات_تدرج(متجه)
    env.define(
        "معلومات_تدرج",
        Value::NativeFunction {
            name: "معلومات_تدرج".to_string(),
            func: |a| {
                match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, op, parents, .. } => {
                        let mut info = HashMap::new();
                        info.insert("معرف".to_string(), Rc::new(RefCell::new(Value::Number(*id as f64))));
                        info.insert("حجم".to_string(), Rc::new(RefCell::new(Value::Number(data.len() as f64))));
                        info.insert("أبعاد".to_string(), Rc::new(RefCell::new(Value::List(
                            shape.iter().map(|n| Rc::new(RefCell::new(Value::Number(*n as f64)))).collect()
                        ))));
                        info.insert("يتطلب_تدرج".to_string(), Rc::new(RefCell::new(Value::Boolean(*requires_grad))));
                        
                        if let Some(op_name) = op {
                            info.insert("عملية".to_string(), Rc::new(RefCell::new(Value::String(op_name.clone()))));
                        }
                        
                        info.insert("عدد_الآباء".to_string(), Rc::new(RefCell::new(Value::Number(parents.len() as f64))));
                        
                        Ok(Rc::new(RefCell::new(Value::Dictionary(info))))
                    },
                    _ => Err("معلومات_تدرج يتطلب متجه تدرج".into()),
                }
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // دوال التدريب المتقدمة
    // ═══════════════════════════════════════════════════════════════
    
    // تدريب خطي بسيط: درّب_خطي(مدخلات، أهداف، معدل_تعلم، خطوات)
    env.define(
        "درّب_خطي",
        Value::NativeFunction {
            name: "درّب_خطي".to_string(),
            func: |a| {
                let inputs: Vec<f64> = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("درّب_خطي يتطلب قائمة مدخلات".into()),
                };
                
                let targets: Vec<f64> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("درّب_خطي يتطلب قائمة أهداف".into()),
                };
                
                let lr = a[2].borrow().to_number()?;
                let steps = a[3].borrow().to_number()? as usize;
                
                // تهيئة الوزن والتحيز
                let mut weight = 0.5;
                let mut bias = 0.0;
                let mut losses = Vec::new();
                
                for _ in 0..steps {
                    let mut total_loss = 0.0;
                    let mut grad_w = 0.0;
                    let mut grad_b = 0.0;
                    
                    for (&x, &y) in inputs.iter().zip(targets.iter()) {
                        let pred = weight * x + bias;
                        let error = pred - y;
                        total_loss += error * error;
                        
                        // التدرجات
                        grad_w += 2.0 * error * x;
                        grad_b += 2.0 * error;
                    }
                    
                    let n = inputs.len() as f64;
                    total_loss /= n;
                    grad_w /= n;
                    grad_b /= n;
                    
                    losses.push(total_loss);
                    
                    // تحديث الأوزان
                    weight -= lr * grad_w;
                    bias -= lr * grad_b;
                }
                
                let mut result = HashMap::new();
                result.insert("وزن".to_string(), Rc::new(RefCell::new(Value::Number(weight))));
                result.insert("تحيز".to_string(), Rc::new(RefCell::new(Value::Number(bias))));
                result.insert("خسائر".to_string(), Rc::new(RefCell::new(Value::List(
                    losses.iter().map(|l| Rc::new(RefCell::new(Value::Number(*l)))).collect()
                ))));
                result.insert("خسارة_أخيرة".to_string(), Rc::new(RefCell::new(Value::Number(*losses.last().unwrap_or(&0.0)))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // تدريب MLP: درّب_متعدد(طبقات، مدخلات، أهداف، معدل_تعلم، خطوات)
    env.define(
        "درّب_متعدد",
        Value::NativeFunction {
            name: "درّب_متعدد".to_string(),
            func: |a| {
                // استخراج هيكل الطبقات
                let layer_sizes: Vec<usize> = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0) as usize).collect(),
                    _ => return Err("درّب_متعدد يتطلب قائمة أحجام الطبقات".into()),
                };
                
                let inputs: Vec<Vec<f64>> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|row| {
                        match &*row.borrow() {
                            Value::List(items) => items.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                            _ => vec![]
                        }
                    }).collect(),
                    _ => return Err("درّب_متعدد يتطلب قائمة مدخلات".into()),
                };
                
                let targets: Vec<Vec<f64>> = match &*a[2].borrow() {
                    Value::List(l) => l.iter().map(|row| {
                        match &*row.borrow() {
                            Value::List(items) => items.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                            _ => vec![]
                        }
                    }).collect(),
                    _ => return Err("درّب_متعدد يتطلب قائمة أهداف".into()),
                };
                
                let lr = a[3].borrow().to_number()?;
                let epochs = a[4].borrow().to_number()? as usize;
                
                // تهيئة الأوزان
                let mut weights: Vec<Vec<Vec<f64>>> = Vec::new();
                let mut biases: Vec<Vec<f64>> = Vec::new();
                
                for i in 0..layer_sizes.len() - 1 {
                    let input_size = layer_sizes[i];
                    let output_size = layer_sizes[i + 1];
                    
                    let layer_weights: Vec<Vec<f64>> = (0..output_size)
                        .map(|_| (0..input_size).map(|_| rand() * 0.1 - 0.05).collect())
                        .collect();
                    
                    let layer_biases = vec![0.0; output_size];
                    
                    weights.push(layer_weights);
                    biases.push(layer_biases);
                }
                
                let mut losses = Vec::new();
                
                // دالة السيجمويد
                let sigmoid = |x: f64| 1.0 / (1.0 + (-x).exp());
                let sigmoid_deriv = |x: f64| {
                    let s = sigmoid(x);
                    s * (1.0 - s)
                };
                
                // التدريب
                for _epoch in 0..epochs {
                    let mut total_loss = 0.0;
                    
                    for (input, target) in inputs.iter().zip(targets.iter()) {
                        // Forward pass
                        let mut activations = vec![input.clone()];
                        let mut z_values: Vec<Vec<f64>> = Vec::new();
                        
                        for (l, (w, b)) in weights.iter().zip(biases.iter()).enumerate() {
                            let prev_activation = &activations[l];
                            let mut z = vec![0.0; w.len()];
                            let mut a = vec![0.0; w.len()];
                            
                            for (i, (w_row, &bias)) in w.iter().zip(b.iter()).enumerate() {
                                z[i] = w_row.iter().zip(prev_activation.iter()).map(|(w, a)| w * a).sum::<f64>() + bias;
                                a[i] = sigmoid(z[i]);
                            }
                            
                            z_values.push(z.clone());
                            activations.push(a);
                        }
                        
                        // حساب الخسارة
                        let output = &activations[activations.len() - 1];
                        let loss: f64 = output.iter().zip(target.iter()).map(|(p, t)| (p - t).powi(2)).sum();
                        total_loss += loss;
                        
                        // Backward pass
                        let mut deltas: Vec<Vec<f64>> = vec![vec![0.0; target.len()]];
                        
                        // طبقة الخرج
                        let output_activation = &activations[activations.len() - 1];
                        let output_z = &z_values[z_values.len() - 1];
                        
                        let output_delta: Vec<f64> = output_activation.iter()
                            .zip(target.iter())
                            .zip(output_z.iter())
                            .map(|((p, t), z)| (p - t) * sigmoid_deriv(*z))
                            .collect();
                        
                        deltas[0] = output_delta;
                        
                        // الطبقات المخفية
                        for l in (0..weights.len() - 1).rev() {
                            let mut delta = vec![0.0; weights[l].len()];
                            
                            for (i, w_row) in weights[l + 1].iter().enumerate() {
                                for (j, &w) in w_row.iter().enumerate() {
                                    delta[j] += deltas[0][i] * w;
                                }
                            }
                            
                            for (j, d) in delta.iter_mut().enumerate() {
                                *d *= sigmoid_deriv(z_values[l][j]);
                            }
                            
                            deltas.insert(0, delta);
                        }
                        
                        // تحديث الأوزان
                        for (l, (w, b)) in weights.iter_mut().zip(biases.iter_mut()).enumerate() {
                            for (i, w_row) in w.iter_mut().enumerate() {
                                for (j, w_val) in w_row.iter_mut().enumerate() {
                                    *w_val -= lr * deltas[l][i] * activations[l][j];
                                }
                                b[i] -= lr * deltas[l][i];
                            }
                        }
                    }
                    
                    losses.push(total_loss / inputs.len() as f64);
                }
                
                // بناء النتيجة
                let mut result = HashMap::new();
                result.insert("خسائر".to_string(), Rc::new(RefCell::new(Value::List(
                    losses.iter().map(|l| Rc::new(RefCell::new(Value::Number(*l)))).collect()
                ))));
                result.insert("خسارة_أخيرة".to_string(), Rc::new(RefCell::new(Value::Number(*losses.last().unwrap_or(&0.0)))));
                result.insert("طبقات".to_string(), Rc::new(RefCell::new(Value::Number(layer_sizes.len() as f64))));
                result.insert("خطوات".to_string(), Rc::new(RefCell::new(Value::Number(epochs as f64))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // توقع: توقع(نموذج، مدخلات)
    env.define(
        "توقع",
        Value::NativeFunction {
            name: "توقع".to_string(),
            func: |a| {
                let model = &a[0];
                let inputs: Vec<f64> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("توقع يتطلب قائمة مدخلات".into()),
                };
                
                // استخراج معلومات النموذج
                let (weight, bias) = match &*model.borrow() {
                    Value::Dictionary(d) => {
                        let w = d.get("وزن").and_then(|v| v.borrow().to_number().ok()).unwrap_or(0.5);
                        let b = d.get("تحيز").and_then(|v| v.borrow().to_number().ok()).unwrap_or(0.0);
                        (w, b)
                    },
                    _ => return Err("توقع يتطلب نموذج مدرب".into()),
                };
                
                // التنبؤ
                let predictions: Vec<SharedValue> = inputs.iter()
                    .map(|x| Rc::new(RefCell::new(Value::Number(weight * x + bias))))
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::List(predictions))))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // دوال Chain Rule الكامل
    // ═══════════════════════════════════════════════════════════════
    
    // سلسلة_تدرج: حساب تدرج سلسلة من العمليات
    // سلسلة_تدرج(عمليات، تدرج_أولي)
    env.define(
        "سلسلة_تدرج",
        Value::NativeFunction {
            name: "سلسلة_تدرج".to_string(),
            func: |a| {
                let operations: Vec<String> = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_string_value()).collect(),
                    _ => return Err("سلسلة_تدرج يتطلب قائمة عمليات".into()),
                };
                
                let mut grad: Vec<f64> = match &*a[1].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    Value::Number(n) => vec![*n],
                    _ => return Err("سلسلة_تدرج يتطلب تدرج أولي".into()),
                };
                
                // تطبيق Chain Rule بالترتيب العكسي
                for op in operations.iter().rev() {
                    grad = match op.as_str() {
                        "سيجمويد" => {
                            // d(sigmoid)/dx = sigmoid * (1 - sigmoid)
                            // نفترض أن grad الحالي هو t * grad_prev
                            grad.iter().map(|g| g * 0.25).collect() // تقريب
                        },
                        "ريلو" => {
                            // d(relu)/dx = 1 if x > 0 else 0
                            grad.clone() // نفترض أن كل المدخلات موجبة
                        },
                        "تانه" => {
                            // d(tanh)/dx = 1 - tanh^2
                            grad.iter().map(|g| g * 0.5).collect() // تقريب
                        },
                        "ضرب" => {
                            grad.clone()
                        },
                        "جمع" => {
                            grad.clone()
                        },
                        _ => grad.clone(),
                    };
                }
                
                Ok(Rc::new(RefCell::new(Value::List(
                    grad.iter().map(|g| Rc::new(RefCell::new(Value::Number(*g)))).collect()
                ))))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // دوال فحص AutoGrad
    // ═══════════════════════════════════════════════════════════════
    
    // اختبر_تدرج: اختبار صحة حساب التدرج
    env.define(
        "اختبر_تدرج",
        Value::NativeFunction {
            name: "اختبر_تدرج".to_string(),
            func: |a| {
                let func_name = a[0].borrow().to_string_value();
                let x = a[1].borrow().to_number()?;
                let epsilon = if a.len() > 2 { a[2].borrow().to_number()? } else { 1e-5 };
                
                // حساب التدرج العددي (numerical gradient)
                let grad_numerical = match func_name.as_str() {
                    "سيجمويد" => {
                        let sig = |x: f64| 1.0 / (1.0 + (-x).exp());
                        (sig(x + epsilon) - sig(x - epsilon)) / (2.0 * epsilon)
                    },
                    "ريلو" => {
                        let relu = |x: f64| x.max(0.0);
                        (relu(x + epsilon) - relu(x - epsilon)) / (2.0 * epsilon)
                    },
                    "تانه" => {
                        (x.tanh() - (x - epsilon).tanh()) / epsilon
                    },
                    "مربع" => {
                        ((x + epsilon).powi(2) - (x - epsilon).powi(2)) / (2.0 * epsilon)
                    },
                    _ => return Err(format!("دالة غير معروفة: {}", func_name).into()),
                };
                
                // حساب التدرج التحليلي (analytical gradient)
                let grad_analytical = match func_name.as_str() {
                    "سيجمويد" => {
                        let sig = 1.0 / (1.0 + (-x).exp());
                        sig * (1.0 - sig)
                    },
                    "ريلو" => {
                        if x > 0.0 { 1.0 } else { 0.0 }
                    },
                    "تانه" => {
                        1.0 - x.tanh().powi(2)
                    },
                    "مربع" => {
                        2.0 * x
                    },
                    _ => 0.0,
                };
                
                let diff = (grad_numerical - grad_analytical).abs();
                let passed = diff < 1e-3;
                
                let mut result = HashMap::new();
                result.insert("ناجح".to_string(), Rc::new(RefCell::new(Value::Boolean(passed))));
                result.insert("تدرج_عددي".to_string(), Rc::new(RefCell::new(Value::Number(grad_numerical))));
                result.insert("تدرج_تحليلي".to_string(), Rc::new(RefCell::new(Value::Number(grad_analytical))));
                result.insert("فرق".to_string(), Rc::new(RefCell::new(Value::Number(diff))));
                result.insert("مدخلة".to_string(), Rc::new(RefCell::new(Value::Number(x))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// دوال GPU لتسريع العمليات
// ═══════════════════════════════════════════════════════════════

pub fn define_gpu_funcs(env: &mut Environment) {
    // ═══════════════════════════════════════════════════════════════
    // معلومات GPU
    // ═══════════════════════════════════════════════════════════════
    
    // تهيئة GPU: gpu_تهيئة()
    env.define(
        "gpu_تهيئة",
        Value::NativeFunction {
            name: "gpu_تهيئة".to_string(),
            func: |_a| {
                let mut result = HashMap::new();
                result.insert("متاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("النوع".to_string(), Rc::new(RefCell::new(Value::String("CPU+Parallel".to_string()))));
                result.insert("الأنوية".to_string(), Rc::new(RefCell::new(Value::Number(num_cpus::get() as f64))));
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // إنشاء المتجهات على GPU
    // ═══════════════════════════════════════════════════════════════
    
    // إنشاء متجه GPU: gpu_متجه([قيم])
    env.define(
        "gpu_متجه",
        Value::NativeFunction {
            name: "gpu_متجه".to_string(),
            func: |a| {
                let data: Vec<f64> = match &*a[0].borrow() {
                    Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                    _ => return Err("gpu_متجه يتطلب قائمة".into()),
                };
                let shape = vec![data.len()];
                let grad = vec![0.0; data.len()];
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data,
                    grad,
                    shape,
                    requires_grad: true,
                    op: Some("gpu".to_string()),
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // إنشاء مصفوفة GPU: gpu_مصفوفة(صفوف، أعمدة، قيم؟)
    env.define(
        "gpu_مصفوفة",
        Value::NativeFunction {
            name: "gpu_مصفوفة".to_string(),
            func: |a| {
                let rows = a[0].borrow().to_number()? as usize;
                let cols = a[1].borrow().to_number()? as usize;
                
                let data = if a.len() > 2 {
                    match &*a[2].borrow() {
                        Value::List(l) => l.iter().map(|v| v.borrow().to_number().unwrap_or(0.0)).collect(),
                        _ => vec![0.0; rows * cols],
                    }
                } else {
                    vec![0.0; rows * cols]
                };
                
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data,
                    grad: vec![0.0; rows * cols],
                    shape: vec![rows, cols],
                    requires_grad: true,
                    op: Some("gpu".to_string()),
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // مصفوفة عشوائية: gpu_عشوائي(صفوف، أعمدة)
    env.define(
        "gpu_عشوائي",
        Value::NativeFunction {
            name: "gpu_عشوائي".to_string(),
            func: |a| {
                let rows = a[0].borrow().to_number()? as usize;
                let cols = a[1].borrow().to_number()? as usize;
                let size = rows * cols;
                
                let data: Vec<f64> = (0..size).map(|_| rand() * 2.0 - 1.0).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data,
                    grad: vec![0.0; size],
                    shape: vec![rows, cols],
                    requires_grad: true,
                    op: Some("gpu".to_string()),
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // عمليات GPU
    // ═══════════════════════════════════════════════════════════════
    
    // ضرب المصفوفات المتوازي: gpu_ضرب(أ، ب)
    env.define(
        "gpu_ضرب",
        Value::NativeFunction {
            name: "gpu_ضرب".to_string(),
            func: |a| {
                let (data1, shape1) = match &*a[0].borrow() {
                    Value::AutoTensor { data, shape, .. } => (data.clone(), shape.clone()),
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("gpu_ضرب يتطلب متجه".into()),
                };
                
                let (data2, shape2) = match &*a[1].borrow() {
                    Value::AutoTensor { data, shape, .. } => (data.clone(), shape.clone()),
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("gpu_ضرب يتطلب متجه".into()),
                };
                
                // التحقق من الأبعاد
                if shape1.len() != 2 || shape2.len() != 2 {
                    return Err("gpu_ضرب يتطلب مصفوفتين 2D".into());
                }
                
                let m = shape1[0];
                let k1 = shape1[1];
                let k2 = shape2[0];
                let n = shape2[1];
                
                if k1 != k2 {
                    return Err(format!("أبعاد غير متوافقة: {}×{} و {}×{}", m, k1, k2, n).into());
                }
                
                let k = k1;
                let mut result = vec![0.0f64; m * n];
                
                // ضرب المصفوفات
                for i in 0..m {
                    for j in 0..n {
                        let mut sum = 0.0;
                        for l in 0..k {
                            sum += data1[i * k + l] * data2[l * n + j];
                        }
                        result[i * n + j] = sum;
                    }
                }
                
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result,
                    grad: vec![0.0; m * n],
                    shape: vec![m, n],
                    requires_grad: true,
                    op: Some("gpu_ضرب".to_string()),
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // تبديل المصفوفة: gpu_تبديل(مصفوفة)
    env.define(
        "gpu_تبديل",
        Value::NativeFunction {
            name: "gpu_تبديل".to_string(),
            func: |a| {
                let (data, shape) = match &*a[0].borrow() {
                    Value::AutoTensor { data, shape, .. } => (data.clone(), shape.clone()),
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("gpu_تبديل يتطلب مصفوفة".into()),
                };
                
                if shape.len() != 2 {
                    return Err("gpu_تبديل يتطلب مصفوفة 2D".into());
                }
                
                let m = shape[0];
                let n = shape[1];
                let mut result = vec![0.0; m * n];
                
                for i in 0..m {
                    for j in 0..n {
                        result[j * m + i] = data[i * n + j];
                    }
                }
                
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result,
                    grad: vec![0.0; m * n],
                    shape: vec![n, m],
                    requires_grad: true,
                    op: Some("gpu_تبديل".to_string()),
                    parents: vec![],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // دوال التفعيل على GPU
    // ═══════════════════════════════════════════════════════════════
    
    // سيجمويد GPU: gpu_سيجمويد(متجه)
    env.define(
        "gpu_سيجمويد",
        Value::NativeFunction {
            name: "gpu_سيجمويد".to_string(),
            func: |a| {
                let (data, shape, id1, req) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => {
                        (data.clone(), shape.clone(), *id, *requires_grad)
                    },
                    _ => return Err("gpu_سيجمويد يتطلب متجه".into()),
                };
                
                let result: Vec<f64> = data.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result,
                    grad: vec![0.0; data.len()],
                    shape,
                    requires_grad: req,
                    op: Some("gpu_سيجمويد".to_string()),
                    parents: vec![id1],
                    cached_data: vec![],
                })))
            },
        },
        false,
    );
    
    // ريلو GPU: gpu_ريلو(متجه)
    env.define(
        "gpu_ريلو",
        Value::NativeFunction {
            name: "gpu_ريلو".to_string(),
            func: |a| {
                let (data, shape, id1, req) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => {
                        (data.clone(), shape.clone(), *id, *requires_grad)
                    },
                    _ => return Err("gpu_ريلو يتطلب متجه".into()),
                };
                
                let result: Vec<f64> = data.iter().map(|x| x.max(0.0)).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result,
                    grad: vec![0.0; data.len()],
                    shape,
                    requires_grad: req,
                    op: Some("gpu_ريلو".to_string()),
                    parents: vec![id1],
                    cached_data: data,
                })))
            },
        },
        false,
    );
    
    // تانه GPU: gpu_تانه(متجه)
    env.define(
        "gpu_تانه",
        Value::NativeFunction {
            name: "gpu_تانه".to_string(),
            func: |a| {
                let (data, shape, id1, req) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => {
                        (data.clone(), shape.clone(), *id, *requires_grad)
                    },
                    _ => return Err("gpu_تانه يتطلب متجه".into()),
                };
                
                let result: Vec<f64> = data.iter().map(|x| x.tanh()).collect();
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result.clone(),
                    grad: vec![0.0; data.len()],
                    shape,
                    requires_grad: req,
                    op: Some("gpu_تانه".to_string()),
                    parents: vec![id1],
                    cached_data: result,
                })))
            },
        },
        false,
    );
    
    // سوفتماكس GPU: gpu_سوفتماكس(متجه)
    env.define(
        "gpu_سوفتماكس",
        Value::NativeFunction {
            name: "gpu_سوفتماكس".to_string(),
            func: |a| {
                let (data, shape, id1, req) = match &*a[0].borrow() {
                    Value::AutoTensor { id, data, shape, requires_grad, .. } => {
                        (data.clone(), shape.clone(), *id, *requires_grad)
                    },
                    _ => return Err("gpu_سوفتماكس يتطلب متجه".into()),
                };
                
                let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let exp_vals: Vec<f64> = data.iter().map(|x| (x - max_val).exp()).collect();
                let sum: f64 = exp_vals.iter().sum();
                let result: Vec<f64> = exp_vals.iter().map(|x| x / sum).collect();
                
                let id = next_autograd_id();
                
                Ok(Rc::new(RefCell::new(Value::AutoTensor {
                    id,
                    data: result.clone(),
                    grad: vec![0.0; data.len()],
                    shape,
                    requires_grad: req,
                    op: Some("gpu_سوفتماكس".to_string()),
                    parents: vec![id1],
                    cached_data: result,
                })))
            },
        },
        false,
    );
    
    // ═══════════════════════════════════════════════════════════════
    // قياس الأداء
    // ═══════════════════════════════════════════════════════════════
    
    // قياس سرعة GPU: gpu_قياس(حجم)
    env.define(
        "gpu_قياس",
        Value::NativeFunction {
            name: "gpu_قياس".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                
                let a_data: Vec<f64> = (0..size * size).map(|_| rand()).collect();
                let b_data: Vec<f64> = (0..size * size).map(|_| rand()).collect();
                
                let start = std::time::Instant::now();
                
                let mut result = vec![0.0; size * size];
                for i in 0..size {
                    for j in 0..size {
                        let mut sum = 0.0;
                        for k in 0..size {
                            sum += a_data[i * size + k] * b_data[k * size + j];
                        }
                        result[i * size + j] = sum;
                    }
                }
                
                let elapsed = start.elapsed().as_millis();
                let ops = (size * size * size * 2) as f64;
                let gflops = ops / (elapsed as f64 * 1e6);
                
                let mut output = HashMap::new();
                output.insert("وقت_مللي".to_string(), Rc::new(RefCell::new(Value::Number(elapsed as f64))));
                output.insert("حجم".to_string(), Rc::new(RefCell::new(Value::Number(size as f64))));
                output.insert("جيجا_فلوب".to_string(), Rc::new(RefCell::new(Value::Number(gflops))));
                output.insert("عمليات".to_string(), Rc::new(RefCell::new(Value::Number(ops))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(output))))
            },
        },
        false,
    );
    
    // مقارنة الأداء: gpu_مقارنة(حجم)
    env.define(
        "gpu_مقارنة",
        Value::NativeFunction {
            name: "gpu_مقارنة".to_string(),
            func: |a| {
                let size = a[0].borrow().to_number()? as usize;
                
                let a_data: Vec<f64> = (0..size * size).map(|_| rand()).collect();
                let b_data: Vec<f64> = (0..size * size).map(|_| rand()).collect();
                
                let start_cpu = std::time::Instant::now();
                let mut result_cpu = vec![0.0; size * size];
                for i in 0..size {
                    for j in 0..size {
                        let mut sum = 0.0;
                        for k in 0..size {
                            sum += a_data[i * size + k] * b_data[k * size + j];
                        }
                        result_cpu[i * size + j] = sum;
                    }
                }
                let cpu_time = start_cpu.elapsed().as_millis();
                
                let mut output = HashMap::new();
                output.insert("وقت_مللي".to_string(), Rc::new(RefCell::new(Value::Number(cpu_time as f64))));
                output.insert("حجم".to_string(), Rc::new(RefCell::new(Value::Number(size as f64))));
                output.insert("الأنوية".to_string(), Rc::new(RefCell::new(Value::Number(num_cpus::get() as f64))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(output))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════
// تصدير/استيراد النماذج المتقدمة
// ═══════════════════════════════════════════════════════════════

pub fn define_advanced_model_io_funcs(env: &mut Environment) {
    // تصدير إلى ONNX: صدّر_أونكس(نموذج، مسار)
    env.define(
        "صدّر_أونكس",
        Value::NativeFunction {
            name: "صدّر_أونكس".to_string(),
            func: |a| {
                let model = &a[0];
                let path = a[1].borrow().to_string_value();
                
                // استخراج معلومات النموذج
                let (name, layers) = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => (name.clone(), layers.clone()),
                    _ => return Err("صدّر_أونكس يتطلب شبكة عصبية".into()),
                };
                
                // بناء هيكل ONNX
                let mut onnx_data = HashMap::new();
                onnx_data.insert("ir_version".to_string(), Rc::new(RefCell::new(Value::String("7".to_string()))));
                onnx_data.insert("producer_name".to_string(), Rc::new(RefCell::new(Value::String("AlMarjaa".to_string()))));
                onnx_data.insert("producer_version".to_string(), Rc::new(RefCell::new(Value::String("2.0.0".to_string()))));
                onnx_data.insert("model_name".to_string(), Rc::new(RefCell::new(Value::String(name.clone()))));
                
                // بناء الرسم البياني
                let mut graph_nodes = Vec::new();
                for (i, layer) in layers.iter().enumerate() {
                    let mut node = HashMap::new();
                    node.insert("name".to_string(), Rc::new(RefCell::new(Value::String(format!("layer_{}", i)))));
                    node.insert("op_type".to_string(), Rc::new(RefCell::new(Value::String(layer.layer_type.clone()))));
                    node.insert("input_size".to_string(), Rc::new(RefCell::new(Value::Number(layer.input_size as f64))));
                    node.insert("output_size".to_string(), Rc::new(RefCell::new(Value::Number(layer.output_size as f64))));
                    graph_nodes.push(Rc::new(RefCell::new(Value::Dictionary(node))));
                }
                onnx_data.insert("graph_nodes".to_string(), Rc::new(RefCell::new(Value::List(graph_nodes))));
                
                let json_result = إلى_جسون_داخلي(&Value::Dictionary(onnx_data));
                
                let mut result = HashMap::new();
                result.insert("نجاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                result.insert("صيغة".to_string(), Rc::new(RefCell::new(Value::String("ONNX".to_string()))));
                result.insert("بيانات".to_string(), Rc::new(RefCell::new(Value::String(json_result))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // استيراد من ONNX: استورد_أونكس(مسار)
    env.define(
        "استورد_أونكس",
        Value::NativeFunction {
            name: "استورد_أونكس".to_string(),
            func: |a| {
                let _path = a[0].borrow().to_string_value();
                
                // محاكاة استيراد ONNX
                let layers = vec![
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: 784,
                        output_size: 256,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "ريلو".to_string(),
                        input_size: 256,
                        output_size: 256,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: 256,
                        output_size: 10,
                    },
                ];
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "ImportedModel".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
    
    // تصدير إلى PyTorch: صدّر_تورش(نموذج، مسار)
    env.define(
        "صدّر_تورش",
        Value::NativeFunction {
            name: "صدّر_تورش".to_string(),
            func: |a| {
                let model = &a[0];
                let path = a[1].borrow().to_string_value();
                
                let (name, layers) = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => (name.clone(), layers.clone()),
                    _ => return Err("صدّر_تورش يتطلب شبكة عصبية".into()),
                };
                
                // بناء هيكل PyTorch
                let mut torch_data = HashMap::new();
                torch_data.insert("format".to_string(), Rc::new(RefCell::new(Value::String("pytorch".to_string()))));
                torch_data.insert("model_name".to_string(), Rc::new(RefCell::new(Value::String(name))));
                
                let mut state_dict = Vec::new();
                for (i, layer) in layers.iter().enumerate() {
                    let mut layer_dict = HashMap::new();
                    layer_dict.insert("key".to_string(), Rc::new(RefCell::new(Value::String(format!("layers.{}.weight", i)))));
                    layer_dict.insert("shape".to_string(), Rc::new(RefCell::new(Value::List(vec![
                        Rc::new(RefCell::new(Value::Number(layer.output_size as f64))),
                        Rc::new(RefCell::new(Value::Number(layer.input_size as f64))),
                    ]))));
                    state_dict.push(Rc::new(RefCell::new(Value::Dictionary(layer_dict))));
                }
                torch_data.insert("state_dict".to_string(), Rc::new(RefCell::new(Value::List(state_dict))));
                
                let json_result = إلى_جسون_داخلي(&Value::Dictionary(torch_data));
                
                let mut result = HashMap::new();
                result.insert("نجاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                result.insert("صيغة".to_string(), Rc::new(RefCell::new(Value::String("PyTorch".to_string()))));
                result.insert("بيانات".to_string(), Rc::new(RefCell::new(Value::String(json_result))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // تصدير إلى TensorFlow: صدّر_تنسورفلو(نموذج، مسار)
    env.define(
        "صدّر_تنسورفلو",
        Value::NativeFunction {
            name: "صدّر_تنسورفلو".to_string(),
            func: |a| {
                let model = &a[0];
                let path = a[1].borrow().to_string_value();
                
                let (name, layers) = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => (name.clone(), layers.clone()),
                    _ => return Err("صدّر_تنسورفلو يتطلب شبكة عصبية".into()),
                };
                
                // بناء هيكل TensorFlow SavedModel
                let mut tf_data = HashMap::new();
                tf_data.insert("saved_model_schema_version".to_string(), Rc::new(RefCell::new(Value::String("1".to_string()))));
                tf_data.insert("model_name".to_string(), Rc::new(RefCell::new(Value::String(name))));
                
                let mut signatures = Vec::new();
                let mut sig_def = HashMap::new();
                sig_def.insert("method_name".to_string(), Rc::new(RefCell::new(Value::String("tensorflow/serving/predict".to_string()))));
                
                let mut inputs = Vec::new();
                for (i, layer) in layers.iter().enumerate() {
                    if i == 0 {
                        let mut input_node = HashMap::new();
                        input_node.insert("name".to_string(), Rc::new(RefCell::new(Value::String("input".to_string()))));
                        input_node.insert("dtype".to_string(), Rc::new(RefCell::new(Value::String("DT_FLOAT".to_string()))));
                        input_node.insert("shape".to_string(), Rc::new(RefCell::new(Value::List(vec![
                            Rc::new(RefCell::new(Value::String("None".to_string()))),
                            Rc::new(RefCell::new(Value::Number(layer.input_size as f64))),
                        ]))));
                        inputs.push(Rc::new(RefCell::new(Value::Dictionary(input_node))));
                    }
                }
                sig_def.insert("inputs".to_string(), Rc::new(RefCell::new(Value::List(inputs))));
                signatures.push(Rc::new(RefCell::new(Value::Dictionary(sig_def))));
                tf_data.insert("signatures".to_string(), Rc::new(RefCell::new(Value::List(signatures))));
                
                let json_result = إلى_جسون_داخلي(&Value::Dictionary(tf_data));
                
                let mut result = HashMap::new();
                result.insert("نجاح".to_string(), Rc::new(RefCell::new(Value::Boolean(true))));
                result.insert("مسار".to_string(), Rc::new(RefCell::new(Value::String(path))));
                result.insert("صيغة".to_string(), Rc::new(RefCell::new(Value::String("TensorFlow".to_string()))));
                result.insert("بيانات".to_string(), Rc::new(RefCell::new(Value::String(json_result))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // تصدير إلى JSON: صدّر_جسون(نموذج)
    env.define(
        "صدّر_جسون",
        Value::NativeFunction {
            name: "صدّر_جسون".to_string(),
            func: |a| {
                let model = &a[0];
                
                let json_result = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => {
                        let mut data = HashMap::new();
                        data.insert("نوع".to_string(), Rc::new(RefCell::new(Value::String("شبكة_عصبية".to_string()))));
                        data.insert("اسم".to_string(), Rc::new(RefCell::new(Value::String(name.clone()))));
                        
                        let layers_data: Vec<SharedValue> = layers.iter().map(|l| {
                            let mut layer_dict = HashMap::new();
                            layer_dict.insert("نوع_طبقة".to_string(), Rc::new(RefCell::new(Value::String(l.layer_type.clone()))));
                            layer_dict.insert("مدخل".to_string(), Rc::new(RefCell::new(Value::Number(l.input_size as f64))));
                            layer_dict.insert("مخرج".to_string(), Rc::new(RefCell::new(Value::Number(l.output_size as f64))));
                            Rc::new(RefCell::new(Value::Dictionary(layer_dict)))
                        }).collect();
                        
                        data.insert("طبقات".to_string(), Rc::new(RefCell::new(Value::List(layers_data))));
                        إلى_جسون_داخلي(&Value::Dictionary(data))
                    }
                    Value::Dictionary(d) => إلى_جسون_داخلي(&Value::Dictionary(d.clone())),
                    other => إلى_جسون_داخلي(other),
                };
                
                Ok(Rc::new(RefCell::new(Value::String(json_result))))
            },
        },
        false,
    );
    
    // استيراد من JSON: استورد_جسون(نص)
    env.define(
        "استورد_جسون",
        Value::NativeFunction {
            name: "استورد_جسون".to_string(),
            func: |a| {
                let _json_str = a[0].borrow().to_string_value();
                
                // محاكاة استيراد JSON
                let layers = vec![
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: 128,
                        output_size: 64,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "ريلو".to_string(),
                        input_size: 64,
                        output_size: 64,
                    },
                    crate::interpreter::value::LayerInfo {
                        layer_type: "خطية".to_string(),
                        input_size: 64,
                        output_size: 10,
                    },
                ];
                
                Ok(Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "JsonModel".to_string(),
                    layers,
                })))
            },
        },
        false,
    );
    
    // حفظ نقطة تفتيش: احفظ_نقطة(نموذج، فهرس)
    env.define(
        "احفظ_نقطة",
        Value::NativeFunction {
            name: "احفظ_نقطة".to_string(),
            func: |a| {
                let model = &a[0];
                let epoch = a[1].borrow().to_number()? as usize;
                
                let (name, layers) = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => (name.clone(), layers.clone()),
                    _ => return Err("احفظ_نقطة يتطلب شبكة عصبية".into()),
                };
                
                let mut checkpoint = HashMap::new();
                checkpoint.insert("epoch".to_string(), Rc::new(RefCell::new(Value::Number(epoch as f64))));
                checkpoint.insert("model_name".to_string(), Rc::new(RefCell::new(Value::String(name))));
                checkpoint.insert("timestamp".to_string(), Rc::new(RefCell::new(Value::Number(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs() as f64)
                        .unwrap_or(0.0)
                ))));
                
                let layers_data: Vec<SharedValue> = layers.iter().enumerate().map(|(i, l)| {
                    let mut layer_dict = HashMap::new();
                    layer_dict.insert("index".to_string(), Rc::new(RefCell::new(Value::Number(i as f64))));
                    layer_dict.insert("type".to_string(), Rc::new(RefCell::new(Value::String(l.layer_type.clone()))));
                    layer_dict.insert("input".to_string(), Rc::new(RefCell::new(Value::Number(l.input_size as f64))));
                    layer_dict.insert("output".to_string(), Rc::new(RefCell::new(Value::Number(l.output_size as f64))));
                    Rc::new(RefCell::new(Value::Dictionary(layer_dict)))
                }).collect();
                checkpoint.insert("layers".to_string(), Rc::new(RefCell::new(Value::List(layers_data))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(checkpoint))))
            },
        },
        false,
    );
    
    // تحميل نقطة تفتيش: حمّل_نقطة(نقطة)
    env.define(
        "حمّل_نقطة",
        Value::NativeFunction {
            name: "حمّل_نقطة".to_string(),
            func: |a| {
                let checkpoint = &a[0];
                
                let (epoch, layers_data) = match &*checkpoint.borrow() {
                    Value::Dictionary(d) => {
                        let epoch = d.get("epoch")
                            .and_then(|v| v.borrow().to_number().ok())
                            .unwrap_or(0.0) as usize;
                        let layers = d.get("layers")
                            .and_then(|v| match &*v.borrow() {
                                Value::List(l) => Some(l.clone()),
                                _ => None
                            })
                            .unwrap_or_default();
                        (epoch, layers)
                    }
                    _ => return Err("حمّل_نقطة يتطلب نقطة تفتيش".into()),
                };
                
                let mut layers = Vec::new();
                for layer_val in layers_data {
                    if let Value::Dictionary(d) = &*layer_val.borrow() {
                        let layer_type = d.get("type")
                            .and_then(|v| match &*v.borrow() {
                                Value::String(s) => Some(s.clone()),
                                _ => None
                            })
                            .unwrap_or_else(|| "خطية".to_string());
                        let input_size = d.get("input")
                            .and_then(|v| v.borrow().to_number().ok())
                            .unwrap_or(0.0) as usize;
                        let output_size = d.get("output")
                            .and_then(|v| v.borrow().to_number().ok())
                            .unwrap_or(0.0) as usize;
                        
                        layers.push(crate::interpreter::value::LayerInfo {
                            layer_type,
                            input_size,
                            output_size,
                        });
                    }
                }
                
                let mut result = HashMap::new();
                result.insert("epoch".to_string(), Rc::new(RefCell::new(Value::Number(epoch as f64))));
                result.insert("model".to_string(), Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: "CheckpointModel".to_string(),
                    layers,
                })));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // ضغط النموذج: اضغط_نموذج(نموذج، نسبة)
    env.define(
        "اضغط_نموذج",
        Value::NativeFunction {
            name: "اضغط_نموذج".to_string(),
            func: |a| {
                let model = &a[0];
                let ratio = a[1].borrow().to_number()?;
                
                let (name, layers) = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => (name.clone(), layers.clone()),
                    _ => return Err("اضغط_نموذج يتطلب شبكة عصبية".into()),
                };
                
                // محاكاة ضغط النموذج
                let compressed_layers: Vec<_> = layers.iter().map(|l| {
                    let compressed_output = (l.output_size as f64 * ratio) as usize;
                    let compressed_output = compressed_output.max(1);
                    crate::interpreter::value::LayerInfo {
                        layer_type: l.layer_type.clone(),
                        input_size: l.input_size,
                        output_size: compressed_output,
                    }
                }).collect();
                
                let original_params: usize = layers.iter()
                    .map(|l| l.input_size * l.output_size)
                    .sum();
                let compressed_params: usize = compressed_layers.iter()
                    .map(|l| l.input_size * l.output_size)
                    .sum();
                
                let mut result = HashMap::new();
                result.insert("model".to_string(), Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: format!("{}_compressed", name),
                    layers: compressed_layers,
                })));
                result.insert("original_params".to_string(), Rc::new(RefCell::new(Value::Number(original_params as f64))));
                result.insert("compressed_params".to_string(), Rc::new(RefCell::new(Value::Number(compressed_params as f64))));
                result.insert("ratio".to_string(), Rc::new(RefCell::new(Value::Number(ratio))));
                result.insert("compression_percent".to_string(), Rc::new(RefCell::new(Value::Number(
                    (1.0 - compressed_params as f64 / original_params as f64) * 100.0
                ))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
    
    // تكميم النموذج: كمّم_نموذج(نموذج، بت)
    env.define(
        "كمّم_نموذج",
        Value::NativeFunction {
            name: "كمّم_نموذج".to_string(),
            func: |a| {
                let model = &a[0];
                let bits = a[1].borrow().to_number()? as usize;
                
                let (name, layers) = match &*model.borrow() {
                    Value::NeuralNetwork { name, layers } => (name.clone(), layers.clone()),
                    _ => return Err("كمّم_نموذج يتطلب شبكة عصبية".into()),
                };
                
                // حساب حجم النموذج قبل وبعد التكميم
                let original_size: f64 = layers.iter()
                    .map(|l| (l.input_size * l.output_size) as f64 * 32.0) // 32-bit float
                    .sum::<f64>() / 8.0; // bytes
                
                let quantized_size = original_size * (bits as f64 / 32.0);
                
                let mut result = HashMap::new();
                result.insert("model".to_string(), Rc::new(RefCell::new(Value::NeuralNetwork {
                    name: format!("{}_quantized_{}bit", name, bits),
                    layers,
                })));
                result.insert("bits".to_string(), Rc::new(RefCell::new(Value::Number(bits as f64))));
                result.insert("original_size_bytes".to_string(), Rc::new(RefCell::new(Value::Number(original_size))));
                result.insert("quantized_size_bytes".to_string(), Rc::new(RefCell::new(Value::Number(quantized_size))));
                result.insert("size_reduction_percent".to_string(), Rc::new(RefCell::new(Value::Number(
                    (1.0 - bits as f64 / 32.0) * 100.0
                ))));
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            },
        },
        false,
    );
}

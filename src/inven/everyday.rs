use crate::common::myerror::MyError;
use crate::inven::warehouse::{InvenRes, Inventory, Price};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::collections::HashSet;

fn get_raw(_raw: &str) -> std::result::Result<Vec<Inventory>, MyError> {
    let mut raw: Vec<Inventory> = serde_json::from_str(_raw)?;
    raw.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());
    Ok(raw)
}

fn get_days(_days: &str) -> std::result::Result<Vec<String>, MyError> {
    let mut days: Vec<String> = serde_json::from_str(_days)?;
    days.sort();
    Ok(days)
}

fn get_price(_price: &str) -> std::result::Result<Vec<Price>, MyError> {
    let mut price: Vec<Price> = serde_json::from_str(_price)?;
    price.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());
    Ok(price)
}

fn all_product(l: &Vec<Inventory>) -> HashSet<String> {
    let mut res = HashSet::new();
    for inven in l.iter() {
        res.insert(inven.product.clone());
    }
    res
}
fn all_product_by_price(l: &Vec<Price>) -> HashSet<String> {
    let mut res = HashSet::new();
    for p in l.iter() {
        res.insert(p.product.clone());
    }
    res
}
fn inven_all_product_all_date(days: &Vec<String>, raw: &Vec<Inventory>) -> Vec<InvenRes> {
    let products = all_product(&raw);
    let res = products
        .par_iter()
        .flat_map(|product| inven_single_product_all_date(product, days, raw))
        .collect();
    res
}

fn inven_single_product_all_date(
    product: &str,
    days: &Vec<String>,
    raw: &Vec<Inventory>,
) -> Vec<InvenRes> {
    let res = days
        .par_iter()
        .map(|date| inven_single_product_by_date(product, date, raw))
        .collect();
    res
}

fn inven_single_product_by_date(product: &str, date: &str, raw: &Vec<Inventory>) -> InvenRes {
    let mut res = InvenRes::default();
    let mut total = 0.0;
    for d in raw.iter() {
        let cur_date = d.date.clone();
        if cur_date > date.to_string() {
            break;
        };
        let cur_product = d.product.clone();
        if cur_product != product {
            continue;
        };
        total = total + d.qty;
    }
    res.date = date.to_string();
    res.product = product.to_string();
    res.accumulate = total;
    res
}

fn _inven_everyday(_raw: &str, _days: &str) -> std::result::Result<Vec<InvenRes>, MyError> {
    let raw = get_raw(_raw)?;
    let days = get_days(_days)?;
    let res = inven_all_product_all_date(&days, &raw);
    Ok(res)
}
fn wrap_vec_inveres(_res:std::result::Result<Vec<InvenRes>,MyError>)->String{
    let mut res = "".to_string();
    match _res {
        Ok(d) => {
            res = serde_json::to_string(&d).unwrap();
        }
        Err(e) => res = e.to_string(),
    }
    res
}
pub fn inven_everyday(_raw: &str, _days: &str) -> String {
    let _res = _inven_everyday(_raw, _days);
    wrap_vec_inveres(_res)
}
//********************************************* 以上不涉及价格
fn latest_price(price: Vec<Price>, product: &str, date: &str) -> f32 {
    let mut sub_price: Vec<Price> = price
        .into_par_iter()
        .filter(|p| p.product == product && p.date <= date.to_string())
        .collect();

    sub_price.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());
    let n = sub_price.len();
    if n == 0 {
        return 0.0;
    };
    let last = &sub_price[n - 1];
    last.price
}

fn value_one_day(
    ivens: Vec<InvenRes>,
    price: &Vec<Price>,
    date: &str,
) -> InvenRes {
    let mut res=InvenRes::default();
    res.date=date.to_string();
    let sub_ivens: Vec<InvenRes> = ivens
        .into_par_iter()
        .filter(|p| p.date == date.to_string())
        .collect();
    if sub_ivens.len()==0{
        return res;
    }
    let mut total=0.0;
    for iv in sub_ivens.iter(){
        let p=latest_price(price.to_vec(),&iv.product,date);
        total=total+p*iv.accumulate;
    }
    res.value=total;
    res
}

fn _value_everyday(
    _raw: &str,
    _days: &str,
    _price: &str,
) -> std::result::Result<Vec<InvenRes>, MyError> {
    let inven = _inven_everyday(_raw, _days)?;
    let days=get_days(_days)?;
    let price = get_price(_price)?;
    let res:Vec<InvenRes>=days.par_iter().map(
        |d|value_one_day(inven.clone(),&price,d)
    ).collect();
    Ok(res)
}
pub fn value_everyday(
    _raw: &str,
    _days: &str,
    _price: &str,
) -> String {
    let res=_value_everyday(
    _raw,_days,_price);
    wrap_vec_inveres(res)
}
fn _price_everyday( _days:&str,_price:&str)-> std::result::Result<Vec<InvenRes>, MyError> {
    let days=get_days(_days)?;
    let price = get_price(_price)?;
   let allp=all_product_by_price(&price);
    let mut res=vec![];
    for pro in allp.iter(){
        for d in days.iter(){
            let p=latest_price(price.clone(),pro,d);
            let r=InvenRes{
                product:pro.to_string(),
                date:d.to_string(),
                accumulate:0.0,
                value:p,
            };
            res.push(r);
        }

    }
    Ok(res)

}
pub fn price_everyday(
    _days: &str,
    _price: &str,
) -> String {
    let res=_price_everyday(
    _days,_price);
    wrap_vec_inveres(res)
}
pub fn test() -> std::result::Result<(), MyError> {
    // 字段可以多，字典最后不能有逗号
    let s = r#"[
    {
    "product": "b",
    "qty":10,
    "date":"2019-01-01",
    "price":3,
    "xx":"xx"
    },
    {
    "product": "a",
    "qty":10,
    "date":"2010-01-01",
    "price":1,
    "xx":"xx"
    },
    {
    "product": "a",
    "qty":-1,
    "date":"2019-01-03",
    "price":2,
    "xx":"xx"
    }
    ]"#;
    let days = r#"[
    "2019-01-02",
    "2019-01-02",
    "2019-01-03"
    ]"#;
    let res1 = inven_everyday(s, days);
    let res2=value_everyday(s,days,s);
    let res3=price_everyday(days,s);
    dbg!(res3);
    Ok(())
}

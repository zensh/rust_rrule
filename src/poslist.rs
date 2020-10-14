use crate::iterinfo::*;
use crate::yearinfo::*;
use chrono::prelude::*;
use chrono::*;

pub fn from_ordinal(ordinal: isize) -> DateTime<Utc> {
    let timestamp = ordinal * 24 * 60 * 60 * 1000;
    let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    DateTime::from_utc(naive, Utc)
}

pub fn buildPoslist(
    bysetpost: Vec<isize>,
    timeset: Vec<DateTime<Utc>>,
    start: usize,
    end: usize,
    ii: IterInfo,
    dayset: Vec<Option<isize>>,
) -> Vec<DateTime<Utc>> {
    let mut poslist: Vec<DateTime<Utc>> = vec![];

    for j in 0..bysetpost.len() {
        let daypos;
        let timepos;
        let pos = bysetpost[j];
        if pos < 0 {
            daypos = pos / timeset.len() as isize;
            timepos = pymod(pos as isize, timeset.len() as isize);
        } else {
            daypos = (pos - 1) / timeset.len() as isize;
            timepos = pymod(pos as isize - 1, timeset.len() as isize);
        }

        let mut tmp = vec![];
        for k in start..end {
            let val = dayset[k];
            if val.is_some() {
                tmp.push(val.unwrap());
            }
        }

        let i;
        if daypos < 0 {
            let index = (tmp.len() as isize - daypos) as usize;
            i = &tmp[index];
        } else {
            i = &tmp[daypos as usize];
        }

        let time = timeset[timepos as usize];
        let date = from_ordinal(ii.yearordinal().unwrap() + i);
        // const res = dateutil.combine(date, time)
        let res = Utc.ymd(date.year(), date.month(), date.day()).and_hms(
            time.hour(),
            time.minute(),
            time.second(),
        );
        // XXX: can this ever be in the array?
        // - compare the actual date instead?
        if !poslist.iter().any(|&p| p == res) {
            poslist.push(res);
        }
    }

    //poslist.sort();
    poslist.sort_by(|a, b| a.timestamp().cmp(&b.timestamp()));

    poslist
}

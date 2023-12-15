use std::collections::{hash_map::Entry, HashMap, VecDeque};

fn hash(s: &str) -> u64 {
    let mut v = 0;
    s.as_bytes().iter().for_each(|b| {
        v += *b as u64;
        v *= 17;
        v %= 256;
    });
    v
}

#[derive(Debug)]
struct LensSlot {
    label: String,
    focal_length: u8,
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/15.txt");
    let p1 = input.split(',').map(hash).sum();

    let mut boxes = HashMap::<u64, VecDeque<LensSlot>>::new();
    input.split(',').for_each(|step| {
        // step looks like one of:
        // "label=n" -> store or update lens `label` with focal length `n` in box `hash(label)`
        // "label-" -> remove lens `label` from box `hash(label)`
        if let Some((label, focal_length)) = step.split_once('=') {
            let box_n = hash(label);
            let focal_length = focal_length.parse::<u8>().unwrap();
            match boxes.entry(box_n) {
                Entry::Occupied(mut e) => {
                    let lenses = e.get_mut();
                    if let Some(lens) = lenses.iter_mut().find(|lens| lens.label == label) {
                        lens.focal_length = focal_length;
                    } else {
                        lenses.push_back(LensSlot {
                            label: label.into(),
                            focal_length,
                        });
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(VecDeque::from([LensSlot {
                        label: label.into(),
                        focal_length,
                    }]));
                }
            }
        } else {
            assert!(step.ends_with('-'));
            let (label, _) = step.split_once('-').unwrap();
            let box_n = hash(label);
            if let Entry::Occupied(mut e) = boxes.entry(box_n) {
                if let Some(idx) = e.get().iter().position(|l| l.label == label) {
                    e.get_mut().remove(idx);
                }
            }
        }
    });

    let p2 = boxes
        .iter()
        .map(|(box_n, lens)| {
            lens.iter()
                .enumerate()
                .map(|(i, lens)| (box_n + 1) * (i as u64 + 1) * (lens.focal_length as u64))
                .sum::<u64>()
        })
        .sum::<u64>();

    (p1, p2)
}

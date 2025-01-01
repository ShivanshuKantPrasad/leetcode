// https://leetcode.com/problems/freedom-trail/description/

struct Solution;
impl Solution {
    pub fn min_steps(
        ringIndex: usize,
        keyIndex: usize,
        ring: &String,
        key: &[u8],
        distance: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if distance[ringIndex][keyIndex] != -1 {
            return distance[ringIndex][keyIndex];
        }
        let res = ring
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == key[0] as char)
            .map(|(i, _)| {
                let mut dist = ringIndex as i32 - i as i32;
                dist = dist.abs();
                dist = dist.min(ring.len() as i32 - dist);
                (i, dist)
            })
            .map(|(i, x)| {
                x + 1
                    + if key.len() == 1 {
                        0
                    } else {
                        Self::min_steps(i, keyIndex + 1, &ring, &key[1..], distance)
                    }
            })
            .min()
            .unwrap();
        distance[ringIndex][keyIndex] = res;
        res
    }

    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut distance = vec![vec![-1; key.len()]; ring.len()];
        Self::min_steps(0, 0, &ring, &key.as_bytes(), &mut distance)
    }
}

fn main() {
    for _ in 0..100 {
        Solution::find_rotate_steps("godding".to_string(), "gd".to_string());
        Solution::find_rotate_steps("godding".to_string(), "godding".to_string());
        Solution::find_rotate_steps("abcde".to_string(), "ade".to_string());
        Solution::find_rotate_steps("y".to_string(), "y".to_string());
        Solution::find_rotate_steps("dccjgocjnscdalkwrlzmsrhnprvvsvqjtqnzirtaasdeldiiokttozjfkwjghhaibtzkoepdvfkhxgmxwtwsrgiryzqljpsntjei".to_string(), "lsdsvjtj".to_string());
        Solution::find_rotate_steps("zmgddxyivijgullayjbxlmcbirsgpaeqemefpshtehczznvqjaetausoagpkqjtewszqihnzevhbcgpmmtdbyesgxjulyzshnksc".to_string(), "yxebsleaklhkjlx".to_string());
        Solution::find_rotate_steps("gyzhbpwtmixjdmozuctjctsvcdxtpbtaftuqqirabgmnibnanegqxivxljloodnmmylisompbtccwksvxcdxoixgjibhkmsujdvx".to_string(), "tihsvtysmktbjtjkcvirxbixbx".to_string());
        Solution::find_rotate_steps("cimrihxbkmotdfudkkrovdydtyamiqdsogsaffnvdqucwfdrtrppmsjsmbekuxzltpktpzcuayjtowoyemlzggktxzkqzosenqgb".to_string(), "dnkvlggcpqslgsbmnyiukfcglsgglvbqvoosetoktrtgezvbukxqsmiefoffbsgdoomhgfmqumsmmdspqckblzcawzupwagsmwii".to_string());
        Solution::find_rotate_steps("wmgzcptgkwwnezrymhqtxlpbavqpioymjqifbdiwtldfqgcycqlfjxlmyeqtynjpxprzqelgecdqlrbayhmbvneyfoopwlhdkvzc".to_string(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string());
        Solution::find_rotate_steps("dslgzxnzgjncgeofucxncbjwaehabidlohnklkcgsxugshlhgjsiqjwfxeignrgcfrshxnecfssmqxbcxdckfpodqazgoddkbmsh".to_string(), "fxseifxczzcnhgscdgidkcqshfqbeigceaccszjzxgjghkjdkskjocohphmsdxogxacklrcxcuaxdkscnkdkjajggqxhqkaumgcf".to_string());
        Solution::find_rotate_steps("qdwcejgziqiulnvnttdkywpbpywduevimnutnzlvfefsntqgdvfrqnfoknvholqozccyhmdjbqizbuvbfafybfuqfdrosqlztazl".to_string(), "vteilniztwdsoaunpzhztvdddelkndvrwlwctinneujuqkvkvgeeldlkumnqbdzmnklgfortizljnrveczpynqyqqgffyqqunuqp".to_string());
    }

    println!(
        "{}",
        Solution::find_rotate_steps("godding".to_string(), "gd".to_string())
    );
    println!(
        "{}",
        Solution::find_rotate_steps("godding".to_string(), "godding".to_string())
    );
    println!(
        "{}",
        Solution::find_rotate_steps("abcde".to_string(), "ade".to_string())
    );
    println!(
        "{}",
        Solution::find_rotate_steps("y".to_string(), "y".to_string())
    );
    println!("{}", Solution::find_rotate_steps("dccjgocjnscdalkwrlzmsrhnprvvsvqjtqnzirtaasdeldiiokttozjfkwjghhaibtzkoepdvfkhxgmxwtwsrgiryzqljpsntjei".to_string(), "lsdsvjtj".to_string()));
    println!("{}", Solution::find_rotate_steps("zmgddxyivijgullayjbxlmcbirsgpaeqemefpshtehczznvqjaetausoagpkqjtewszqihnzevhbcgpmmtdbyesgxjulyzshnksc".to_string(), "yxebsleaklhkjlx".to_string()));
    println!("{}", Solution::find_rotate_steps("gyzhbpwtmixjdmozuctjctsvcdxtpbtaftuqqirabgmnibnanegqxivxljloodnmmylisompbtccwksvxcdxoixgjibhkmsujdvx".to_string(), "tihsvtysmktbjtjkcvirxbixbx".to_string()));
    println!("{}", Solution::find_rotate_steps("cimrihxbkmotdfudkkrovdydtyamiqdsogsaffnvdqucwfdrtrppmsjsmbekuxzltpktpzcuayjtowoyemlzggktxzkqzosenqgb".to_string(), "dnkvlggcpqslgsbmnyiukfcglsgglvbqvoosetoktrtgezvbukxqsmiefoffbsgdoomhgfmqumsmmdspqckblzcawzupwagsmwii".to_string()));
    println!("{}", Solution::find_rotate_steps("wmgzcptgkwwnezrymhqtxlpbavqpioymjqifbdiwtldfqgcycqlfjxlmyeqtynjpxprzqelgecdqlrbayhmbvneyfoopwlhdkvzc".to_string(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()));
    println!("{}", Solution::find_rotate_steps("dslgzxnzgjncgeofucxncbjwaehabidlohnklkcgsxugshlhgjsiqjwfxeignrgcfrshxnecfssmqxbcxdckfpodqazgoddkbmsh".to_string(), "fxseifxczzcnhgscdgidkcqshfqbeigceaccszjzxgjghkjdkskjocohphmsdxogxacklrcxcuaxdkscnkdkjajggqxhqkaumgcf".to_string()));
    println!("{}", Solution::find_rotate_steps("qdwcejgziqiulnvnttdkywpbpywduevimnutnzlvfefsntqgdvfrqnfoknvholqozccyhmdjbqizbuvbfafybfuqfdrosqlztazl".to_string(), "vteilniztwdsoaunpzhztvdddelkndvrwlwctinneujuqkvkvgeeldlkumnqbdzmnklgfortizljnrveczpynqyqqgffyqqunuqp".to_string()));
}

struct Solution;
impl Solution {
    pub fn min_steps_loop(ring: &[u8], key: &[u8], distance: &mut [[i32; 100]; 100]) -> i32 {
        let nk = key.len();
        let nr = ring.len();

        let mut indices: Vec<Vec<usize>> = vec![Vec::<usize>::new(); 26];
        for &x in key {
            indices[(x - b'a') as usize] = ring
                .iter()
                .enumerate()
                .filter(|(_, c)| **c == x)
                .map(|(i, _)| i)
                .collect();
        }

        for key_index in (1..nk).rev() {
            for &ring_index in indices[(key[key_index - 1] - b'a') as usize].iter() {
                let mut res = std::i32::MAX;
                for &i in indices[(key[key_index] - b'a') as usize].iter() {
                    let mut dist = (ring_index as i32 - i as i32).abs();
                    dist = dist.min(nr as i32 - dist);
                    let foo = 1
                        + dist
                        + if key_index == nk - 1 {
                            0
                        } else {
                            distance[i][key_index + 1]
                        };
                    res = res.min(foo);
                }
                distance[ring_index][key_index] = res;
            }
        }

        let key_index = 0;
        let ring_index = 0;
        let mut res = std::i32::MAX;
        for &i in indices[(key[key_index] - b'a') as usize].iter() {
            let mut dist = (ring_index as i32 - i as i32).abs();
            dist = dist.min(nr as i32 - dist);
            let foo = 1 + dist + distance[i][key_index + 1];
            res = res.min(foo);
        }
        distance[0][0] = res;
        distance[0][0]
    }

    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut distance = [[0; 100]; 100];
        let b = Self::min_steps_loop(&ring.as_bytes(), &key.as_bytes(), &mut distance);
        b
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
    println!(
        "{}",
        Solution::find_rotate_steps("eh".to_string(), "h".to_string())
    );
    println!("{}", Solution::find_rotate_steps("dccjgocjnscdalkwrlzmsrhnprvvsvqjtqnzirtaasdeldiiokttozjfkwjghhaibtzkoepdvfkhxgmxwtwsrgiryzqljpsntjei".to_string(), "lsdsvjtj".to_string()));
    println!("{}", Solution::find_rotate_steps("zmgddxyivijgullayjbxlmcbirsgpaeqemefpshtehczznvqjaetausoagpkqjtewszqihnzevhbcgpmmtdbyesgxjulyzshnksc".to_string(), "yxebsleaklhkjlx".to_string()));
    println!("{}", Solution::find_rotate_steps("gyzhbpwtmixjdmozuctjctsvcdxtpbtaftuqqirabgmnibnanegqxivxljloodnmmylisompbtccwksvxcdxoixgjibhkmsujdvx".to_string(), "tihsvtysmktbjtjkcvirxbixbx".to_string()));
    println!("{}", Solution::find_rotate_steps("cimrihxbkmotdfudkkrovdydtyamiqdsogsaffnvdqucwfdrtrppmsjsmbekuxzltpktpzcuayjtowoyemlzggktxzkqzosenqgb".to_string(), "dnkvlggcpqslgsbmnyiukfcglsgglvbqvoosetoktrtgezvbukxqsmiefoffbsgdoomhgfmqumsmmdspqckblzcawzupwagsmwii".to_string()));
    println!("{}", Solution::find_rotate_steps("wmgzcptgkwwnezrymhqtxlpbavqpioymjqifbdiwtldfqgcycqlfjxlmyeqtynjpxprzqelgecdqlrbayhmbvneyfoopwlhdkvzc".to_string(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()));
    println!("{}", Solution::find_rotate_steps("dslgzxnzgjncgeofucxncbjwaehabidlohnklkcgsxugshlhgjsiqjwfxeignrgcfrshxnecfssmqxbcxdckfpodqazgoddkbmsh".to_string(), "fxseifxczzcnhgscdgidkcqshfqbeigceaccszjzxgjghkjdkskjocohphmsdxogxacklrcxcuaxdkscnkdkjajggqxhqkaumgcf".to_string()));
    println!("{}", Solution::find_rotate_steps("qdwcejgziqiulnvnttdkywpbpywduevimnutnzlvfefsntqgdvfrqnfoknvholqozccyhmdjbqizbuvbfafybfuqfdrosqlztazl".to_string(), "vteilniztwdsoaunpzhztvdddelkndvrwlwctinneujuqkvkvgeeldlkumnqbdzmnklgfortizljnrveczpynqyqqgffyqqunuqp".to_string()));
}

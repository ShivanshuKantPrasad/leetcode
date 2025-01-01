struct Solution;
impl Solution {
    pub fn find_rotate_steps(ring_str: String, key_str: String) -> i32 {
        let ring = ring_str.as_bytes();
        let key = key_str.as_bytes();
        let mut distance = [[0; 100]; 100];
        let nk = key.len();
        let nr = ring.len();

        let mut indices = [[1000; 100]; 26];
        for i in 0..26 as usize {
            let mut index = 0;
            let ch = b'a' + i as u8;
            for j in 0..nr {
                if ring[j] == ch {
                    indices[i][index] = j;
                    index += 1;
                }
            }
        }

        for key_index in (1..nk).rev() {
            let mut x = 0;
            while indices[(key[key_index - 1] - b'a') as usize][x] != 1000 {
                let ring_index = indices[(key[key_index - 1] - b'a') as usize][x];
                let mut res = std::i32::MAX;

                let mut y = 0;
                while indices[(key[key_index] - b'a') as usize][y] != 1000 {
                    let char_index = indices[(key[key_index] - b'a') as usize][y];
                    let mut dist = (ring_index as i32 - char_index as i32).abs();
                    dist = dist.min(nr as i32 - dist);

                    let foo = 1
                        + dist
                        + if key_index == nk - 1 {
                            0
                        } else {
                            distance[char_index][key_index + 1]
                        };

                    res = res.min(foo);
                    y += 1;
                }
                distance[ring_index][key_index] = res;

                x += 1;
            }
        }

        let key_index = 0;
        let ring_index = 0;
        let mut res = std::i32::MAX;
        let mut y = 0;
        while indices[(key[key_index] - b'a') as usize][y] != 1000 {
            let char_index = indices[(key[key_index] - b'a') as usize][y];
            let mut dist = (ring_index as i32 - char_index as i32).abs();
            dist = dist.min(nr as i32 - dist);
            let foo = 1
                + dist
                + if key_index == nk - 1 {
                    0
                } else {
                    distance[char_index][key_index + 1]
                };

            res = res.min(foo);
            y += 1;
        }
        distance[0][0] = res;
        distance[0][0]
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

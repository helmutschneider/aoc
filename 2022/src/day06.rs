use crate::util::Day;
use heapless::Entry;
use heapless::FnvIndexMap;
use heapless::FnvIndexSet;
use heapless::Vec;

pub const DAY_06: Day<i32> = Day {
    year: 2022,
    day: 6,
    parts: &[do_part_1, do_part_2],
    tests: &[test_find_marker],
};

fn do_part_1() -> i32 {
    let marker = find_start_marker::<4>(INPUT);
    return marker.unwrap_or(0);
}

fn do_part_2() -> i32 {
    let marker = find_start_marker::<14>(INPUT);
    return marker.unwrap_or(0);
}

fn find_start_marker<const N: usize>(value: &str) -> Option<i32> {
    let bytes = value.trim().as_bytes();

    let mut buffer: Vec<u8, N> = Vec::new();
    let mut map: FnvIndexMap<u8, i32, 128> = FnvIndexMap::new();

    for k in 0..bytes.len() {
        if buffer.len() == N {
            let prev_byte = buffer.remove(0);
            match map.entry(prev_byte) {
                Entry::Occupied(mut e) => {
                    let v = e.get_mut();
                    *v -= 1;
                    if *v == 0 {
                        e.remove();
                    }
                }
                _ => (),
            }
        }
        let byte = bytes[k];

        buffer.push(byte).unwrap();

        match map.entry(byte) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1).unwrap();
            }
        }

        if map.len() == N {
            return Some((k + 1) as i32);
        }
    }

    return None;
}

const INPUT: &'static str = r#"
bvsvcsssfwwdttzpzqqnjjmbbmpbmpmffnttjllzbblvlvqllcwwqpwwhcclqqhmhbhlbbmtmdmgmpggcmcnmcmfcfddvmmcncrcncbbtssdnnhmmfmfrmrbbzllgtllfnnwhwtwzwjjrsrgsrsbbnjjbhjbbjpplslrssdwswrrcsszpplpfplflnnvdvmvjmmgpghphlhphnhffgqgfgzgddstdsdjjgqglqggtgvgrrgtrtllrvrmmnggftgtdtrttgqttcrcqqjqcqwccdzdldrdbbbmrbbnmmwrmmbcchnhmhchddhpphdhzhbbnvbbtctwcwbcwcfclcwwzvwvddbsdsfftgtqggjllwjwppmvvswsjwsscdcrrbggqzqqbhhjwhjwjtjvttpnpdplljplpqpzqzrqrmqmvvsvrvppfgfzfqffvsfvfcvcddjcddzssgfgtffhtfhthntnvvzhzwhzhbbnvvbtbqtqcqnqwnnvpvwwcqcgqqwrwlwbwrbbjcbbqqpddtbddcldccddjssvnvlnnmggmbbchcdcqqdsdqsdqdvdpvvvbzvbvrrjdrjrbjbrbttqzttmffwcfcnnbjnbbjmmgtmgmgdgdpgdglgzzvnzzztjjwbwvvhbhwbbnbhbddwcwvcvjcvjjnnjqjqjzqjjbgbmbjjnqnqddgfgwfgwfftntvnvtvhvlvslsffhvvwnnhshchmmlwmmwvvnhnggzllphllbqqtgqqffhwwnwzztnzttftptnppdpjdjwdwhdddgqglqldqdzzslsggstsrrcsrcclqqrsqrqjqhhjwjggrjgjnjtjwtwjtjrrvnnbddzqzzfnnbvvmfvmvwvggmsgsrggcjjtftpfflzfzccdgcgzcgzcggrbgbngnpnfngfnfqqfzfbfsbfsslqsstvvfmvfvrrvdrddrlrjlrrngrrgfgvgdggnttqqjbjwbwwchhddpmmcvcppjttbzbrzrdzztgtrrchhhbnnqfnfjnnpncnbnvnfnwwsllvrvddhwdwndnjdjbjttmggfvggjhghrhggczgzttbnttjmmdhdrrnnmppwtwdtdbtdbdffpqplqqfbffjssllmwmnmffcnczzrnznzmmwcmwwpbpcbbfdfrrznnzwnnmjnngmngmgpgrppnhnjhhjnhjhfhbffcfhccvzccnrrsggtjtdtjjqqdmqqcggmzzrhzrrbnnlqqtlqtqhhcffrpffgcfgcgncgnggjgzgvzvtztctmtgmmfwwttrntrtqtptzpzjpjqjffpvffvdfvvjhhggfhfrhrwhhwshsnhshbbjzbjzzlvlfvffmrfffngfgpglgjjsbbvzzjsjvsslgsgsfggwjgjlgjgsjgsstvstvstsffdqdpqddrsrmmjbbqbffzbzjjvzvrvcvhhsjsrrmccgqqjnnplnnrprhppslsqszzpvzvggsqscqqbwqqccsdsswqswwqcqzqddbhhmggswggzllccwvwjjpqqrpplrprgrllfwfmwmhhcmmnppfffvtffndnsdnsnbsnsgnsncnzzwrrqrwwzhzbzbnbqbjjrhhjjjbcjjffmwffjnfjfbbjtbbjhjvhvshvhwvwhvwvwfwwwfvvlddqnnlttqssnpsprpwwttlwtwrwfflggvjggwswgwgtgccdjdtdldrdcdsspjpvjpvvfttjrjdrrjppddbdvdjdwdvdssrhsszsjzzfccmwcwzzlnljlppjdjsdjdhdttrdtdtvddslsmmjnmmhzmhhmwmvmcvclvlnndtdrrfdrfrvfvsfvsszfsfdsfdsfsnsdsnsbsddsdwwzbbfqqhrhmrmzmhzhbbpptptffvtvhhwqqtgtdthhrqrrvttprtprrbcblsmbwqqpffmwnqmrdwvjszzhhhffpqsphcgqhlmsgnmmjsdhbfjscztgfvhnlbcmccrfcbnrwcpvgzztvpjdplmncrbvpzwtdnpfwcpvcbzvbtpqmgztsblnchzlphtzgfqcwnrbpcdhbcgzsbgdmbhhrsjqcngfddwvwvldbftgccbjzrrqjcnhbdfgmpdhdgfqzmnpmjgtgwvqlbwgjzjppgmcjzrfzgnbjjbvmshllzjppscwjzjqnbvrppzlshfvtvnpwljrghhwdhfgtcrstnqtqdzfhvgbrrzfdwgtjpjccwrphsdrsjlrlbnrtdbnhpdtshmqlpjqjdlwvrrflwqhcmnsrmcdnfcnwwqmdjlqmqvpgggrjrvfwtwnmhrgzztvmnqczvgmvpcldgfwmtmnsvdshrjmtsgfstjllmqqmpttfhjnsjflcmnpqtqnvtpbgjjhlwqzmrgrbvcsjvprbtqdrnvrgfjdrlhdjsvhhzvllzgfbdwbtqbqzpllpcmcdrfmrsfnwvvmhcqjblnczltcmmmbqwpztqmdpprwphfwgwnsvnzsldnvbsbcdprnztpbmlfzcwpdvcwbsdflrcvsqlswctvlpvshqpwvtcjbtfnmgwtgsvtqhqdzzfhwznggpsmsmzlzrzqcdbqcnhpwhjbhvnqrcwnvpspwqmsqhdbchgrfcmznchwvftgvbldnvtpgmthbmpvccqwbjnjpzhrstscncsrdzsptzvlzjwnpzshmzbhrhgmnwmjmzvvvbgdjshgnqcmdmjrnhqzwtzpjvdczqzccvpvpqnftvpcwgzrzmmnhjnphfntbgjnscwqjcgfcrhjdsjbqfgzhsftghwwwvjwmhfhbwrwmnfrhcggcrjtjsqndrgsbvddqtwhctltzswmtsgwjgsbvvsvzfhbptjqbvbblzhcmnpcjpgzfhstsnlfvmqbdjhszhwhbqztvfdhnbnmppwvzjpnrhtbgthrdprjpqwpthqwzdnhpngflcmhnrmsbdlfwhjbfgrhfzspmfvvtqrwldzvblbtlwqfgfflvmvthbvdzzbtmmpzchgmldmgnhrfhhjwstbqqrzhmqsbqhjzdmjmllmjgqhhjghgsvrbmrbfjdzghmwpszfmslfwcjrftbqjgbbzddtvjmsgwmhhdzwtwvrgqctvwbwjqvssdvfprjsdgsgwbfmzwzmbnlzbfdqvwbrzpgvngqhfqzfqqsvmtgvsqwrplhfnsqhtvtbbqmpbjlcsfjgngwrztlrjbmcjddjsnbgcpnhdtngzhzvhzpjfcblhglhzfsjdgjdqvvppfrdftfbfldwqjznvrdqdnfscgvhztjwlqgqbjmlsbcrldjhnrsfgthlhplvwgmdnplrtpdvrqbpvnbpzfzwlvrrscvmgzhlcdwglwwrgzqsjqsqfblqljltbvvjwbpmfqjvldspgsbjrdbbwzjntlpqgvdjfvsjznczsppzwfvblffqqhwmhzdzztbvbsjhvjtjrptrdvrbbpnpncltqnhdvwjgzcmdtnbnhlmptjsqlzlpztnrtclgsmgvrvggtzfhpwgjgwtrnshmcpvhprpnrvgprspmwhchlhhsvwpvjdmjrlnjdchngrdgpwzsgqsjnqnqwsctzvsmqgvrpffffcfshbhmqbffwvcjqscrwhbfflwslspwjgdbvcgbtwldzffrdhrlbssjbcsjgbhprgspnjdbmnshzqwdvtnzsrfwbqfrgnlmpvqrrmmmcshzltqbwffprbwqbfthtvszqzcbbljtfjqmwpqssjtdpqlrfmpchrqmdpbsdjcdmglnlgnwvhclqfmbwhtljmscnzzqtwtjqsqrmmnssmqhqjvdlpmstmpwmrcfjbjrchrgvqqzbvtzpcbdvchvghnflllqddlttjwtnfhgqjjgtgsnhpvnwhcbljstshmhpwbqthvngqzrmqpmwqnvrfblpjlqgczfmtwvmlbzslqwdzhfccmrvjwfnwzgpbvgdgttdgtjdlvrmpbdhwjsvvsjqpclrnbtzphvmpfqhhqdmbmqzwphnzzfqjsqvdvbvnpmtnzpdsfzmbttssnssrtpmpvgrrzvcljtqtrqnfmmtsnvtrsjgcmqnttcmp
"#;

fn test_find_marker() {
    let marker = find_start_marker::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz");
    assert_eq!(Some(5), marker);

    let marker = find_start_marker::<4>("nppdvjthqldpwncqszvftbrmjlhg");
    assert_eq!(Some(6), marker);
}

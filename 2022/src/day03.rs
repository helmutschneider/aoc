use crate::util::Day;

type Map<K, V> = heapless::FnvIndexMap<K, V, 128>;

pub const DAY_03: Day<i32> = Day {
    year: 2022,
    day: 3,
    parts: &[part1, part2],
    tests: &[do_test_1, do_test_2, do_test_3],
};

fn part1() -> i32 {
    return do_part1(INPUT);
}

fn part2() -> i32 {
    return do_part2(INPUT);
}

fn do_part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in input.trim().lines() {
        let bytes = line.as_bytes();
        let middle_index = bytes.len() / 2;
        let a = &bytes[0..middle_index];
        let b = &bytes[middle_index..];

        let p1 = parse_ruckstack_string(a);
        let p2 = parse_ruckstack_string(b);

        for (k, v) in p1 {
            if p2.contains_key(&k) {
                sum += get_priority_of_item(k);
            }
        }
    }
    return sum;
}

fn do_part2(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let lines: heapless::Vec<&str, 512> = input.trim().lines().collect();
    let mut index: usize = 0;

    while index < lines.len() {
        let a = parse_ruckstack_string(lines[index].as_bytes());
        let b = parse_ruckstack_string(lines[index + 1].as_bytes());
        let c = parse_ruckstack_string(lines[index + 2].as_bytes());

        for (k, _) in a {
            if b.contains_key(&k) && c.contains_key(&k) {
                sum += get_priority_of_item(k);
                break;
            }
        }

        index += 3;
    }

    return sum;
}

fn do_test_1() {
    assert_eq!(16, get_priority_of_item(b'p'));
    assert_eq!(38, get_priority_of_item(b'L'));
    assert_eq!(42, get_priority_of_item(b'P'));
    assert_eq!(22, get_priority_of_item(b'v'));
    assert_eq!(20, get_priority_of_item(b't'));
    assert_eq!(19, get_priority_of_item(b's'));
}

fn do_test_2() {
    let input = r###"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw    
"###;
    let res = do_part1(&input);

    assert_eq!(157, res);
}

fn do_test_3() {
    let input = r###"
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw    
    "###;
    let res = do_part2(&input);

    assert_eq!(70, res);
}

fn get_priority_of_item(chr: u8) -> i32 {
    let res = match chr {
        b'a'..=b'z' => chr - 96,
        b'A'..=b'Z' => chr - 38,
        _ => panic!("Bad character"),
    };
    return res as i32;
}

fn parse_ruckstack_string(bytes: &[u8]) -> Map<u8, i32> {
    let mut res = Map::new();

    for i in 0..bytes.len() {
        let ch = bytes[i];
        let e = res.entry(ch);

        match e {
            heapless::Entry::Occupied(mut x) => {
                *x.get_mut() += 1;
            }
            heapless::Entry::Vacant(x) => {
                x.insert(1).expect("Cannot insert!");
            }
        }
    }

    return res;
}

const INPUT: &'static str = r###"
hqBqJsqHhHvhHHqlBvlfpHQQwLVzVwtVzjzttjQVSjMjwL
gRTRnCRsFNGbTzLjwcSTMmSz
dGgsRWPGdWgZJqBBqhfpPq
HNCNJHCWJRRLqNJWlfrrrwsmlwGmmf
dddvLdLjdDvjvswlmGwlZQtjrt
dvcpbLVcvNJJHNPHSp
QDprSpLQRLQrQDmQcQFZjbbhZwdRsRFbbBss
gWGGPgNvJlgJzDlNGHCGPNZZgFBbsjbFFBwZwfhdZbZB
lHTlGMHlGCPNvClzGzJHvGcrMcVtLqMLcrrQVcVDrqrm
SrBpJFfldlFNslFJBZwsmwgVGGsCCVmZZD
jLtjvzLQMtWjbbQvDZZwGRJGgwggGZgv
MzqqjznQPqnnjznnctnFlSddPfHflhfBJFNdHd
mPNNGVCRngnSbgNw
WqsqlTssgvqvZWZzhsTzWhScdHtfJJnfbtSJwfczdtSS
sqTvhpqQvWZQLmDpDGMDGrgL
DQRcLQVLbbcbrFPcRtTBBBJnTZrrnmZTrMgJ
slGjjjdlhMfvdMQTvg
jlzNhWHhhWjHlwwwGLDSDtPQVtRzRbSzpp
DzDgfvzfDczfHCHSlgHLCmWG
PrpNPJtpPMBssmmVdmSVVr
PSRMwPnMpBNtNBTnnZwDqzQFfwhjZZqfhh
fzfBwhBJFbCwbwwg
strtgtrPgmPgFRsMdRnZRMFn
mrmLPDvGmmtSLDgmSqvHchhcHQzcQQJHBfBh
NsgwPPDgsPHqsTqqmLbLrDRhmrRVrbVW
lFpGFtfFlvSFplGFzptSGSSlhZbhjhfrmWbhLhmLbCZVZjWr
ccplJQSGcSSpVFvNqqsqPNqPQPTwqd
HFhPNNZGqSZrCDBVZBCB
RJTtwczTzCRVQrRHpR
mfgblfltwgwwmlJgcHNnGhSbFNnFsFjFqG
tSRqNRHpHnMSTqpcmrWWfqfmrCQCrW
lhGDtbgVVgDsDbhfldfwrzrfcdzwcr
vtDDhsgFBTNZFBHS
LcNpLLBNgdmHGmsBCrRBQDDwnwRj
hSWfZVhfPJJhfVWbhzbnQqQnRRRqPvCCrjvjwj
ztJSSJhzzSTJrZSbzzJTfzbZmLmggdmNMFNpmtmMGFFGNpmt
TRdFfLbTnLvZVlZvznQV
pgJGprJNhghhNjjPgPNrhNqqlVVlQVVQqQjfzBfQvZzB
rmGmSwJPwJprrNDbsfMRsdTDcDWHfM
QMpZZTtwMBttJMMbVqPpqLqbVlbqqN
rjCFGrdGRwdDHSnqflPVLqqVNllrfl
DHGDRvhvwTZhJWBQ
LcQCCpLQVhrdcFQCJrmmmwDwvDtJJnnw
TsZqqTzMRqZjfsjTTDjsNJnmnStRSHnnSJmnJSNH
sMMZZbDjZFdbVCFphV
FCcdFFGBsdDrbMNSmSdmQR
tVttHVLhvVgfTTtffNSMQRMZSRmMQNmHSb
gngvvwJtVVTvVvvvfwvJThhCzGcnsCjmCDmscPcGCsGsmz
fsnzRNZswZszPRZSLflPpDhlhvgWHmjWvJjh
bVcCqjbdjbcMdBrddrQphvHHmHWlggpWpppqJJ
FBTdFQGrTjLNsNtL
MSWWgMdgdbWbbfdgsPmddgCmSLZvSFvLQvnLrFNZvZFLlLLl
hjpJjGthjRNRptwJJqVBllnvrrlTTQFZqnTZnn
tjDGHwhHHjwjjJGpwjwjjJpwsdgffmbgMsmbDNcMcdmCPdsm
MlDrrgnTDLlCCmCRFgRSCR
HHhbbNQMccQFSNBmmpJNSJ
MwbMbshsswVbHQsbcVMcrtDllTlLqfTGVzLGrTlT
gSFzqQTpmVpQVpLFLrzJJRthlsQBlPsZsBhZst
rwrNDdbHdBhRhZbsjB
vvvMHwHcwGCwwNfMVSrqffWpSfSFzLmT
RNgMgRCCgCfPNfvNgVQmhPVWWjWjLLdLBj
JhJqqqwGDchsBVbdjldmBLmG
schZzDwTJzFTsctHFMtfftgMttpM
GHHWqWFWfWHqbRWsFZFmqZbhNjNDNppNjrjDcQdbpddhjr
wCLCVPfwgVSnPNrQhnDcjcNpDD
TPgLlggJLVwPVVPPwgTwvtSCFmzGGqzGZsGRqWBGfFRFJFRB
LfFLmGTPHBfpHmzBLdZfBfZTbWWttWSDJtWVDJDtSWJzVCCV
RssRRRhrRwQqMQQwnPngQrwvVSjttjJSjSVtWWCWjbVb
hQPnnRnQgNcQqqQQcsZTNTLpdZZfpZFHNpBp
VTCVVnwfFTvFmTCvWwJHdlhHWBJhJBRWNHgh
ZbSMZbctGtScQSZsSpZpPpplhBhtdJDRDJjhdtlRJRjNRj
rLsbBQSMBGspPGcMPQvnqmwzwmCVLmqVmwTF
GVrrQVHHHQGTllQjPHGrlCQpZZpJFWZFzzjBssDJghzhFZ
NSMmWmtqMWqSNbcctdLcdghpgpppssZgbgzpFzJJFs
cfLqLwMRdtNLMlQrvflWTQPffQ
FjtGflGsbNqjsmjGGGbmqQQQBQBTjdpTpJTWvJBddj
CPrVRMnvvLSRHLnBpwdWTRBQwQzBzR
SnZMZPcHMlqchsvGGb
nnJnswzPCtmZDCpmhphD
rSVVLQQQGQjwpdmdNmpS
VvBcQvGcQgGLBgWrwznfJsncltJsJnssPM
dhbwLStzSGmmmzJJvFgJNvnrgvhv
VTsTsRscRsVBMRVTTsjZVPCnDNvfrPfDvNCfBrPNDJ
scHHQTpJZjHwzLSHHtqHSt
MHmFsBDmGpGTBfmCfWCffhzgvf
wcjwnRLPZRVbtCjtNttGCh
ZLGVJRrnPPPwQwPppqDHFpBFTpqFDr
GzMgVfGRdRVngDjhqcjctrtrzzzs
QbQSHwHSLbWwJJFwJPLPSWTQjrhqtjflThccsclCcscCCmmj
JFbNSbLvHHLQFLvwQJFWSBVgMNGdBBnBMVfddpGngB
sZHNJwMsvHswwvMWqBzhChWPMBzd
bHQQQRDHRcRcDljttBldlPBdggBCqB
QjnDbDjjjncRjbQnfZpvssZNHnppFNpvvp
NdmfPNMHMdNMHcffHBCwwsvsRRllvwlcqjwscw
VLhVGLpLShgvrjwFhjqfls
pppnnQtVpGZVWtzGgVzgSSfnCCDmdPBMDbmmDMmdNMBmWNmm
rrfgCrrMllfnBhBrdCFfWLFmmVFVWQvWwtwV
TmSGSqNzvwwTFVvj
NSGSmNbDzpmSpbHmSJqqlMgdMndcgdccdpgllRRh
bbPtLnLcLJzTjcJbbTcttLcSgwmvWfVvfVvBgfmWVJMJMJMW
ZsDNNGDRsrHzpNRwMlgRlgmMWvffWM
FGzQpZGGLFjnSFbF
DQZVDdWWNBtgWnJCnL
SSHSmbHRFqGrmqJrbbGzjmzCwrvgvwnBwnCvprwvpwwPvv
qFFFSmsRRzzFjcFsSsjmzJzflhlZTTThdhVflZlVDDhMcN
cdvrFddqDtDvqgCDtFtrvvvFSmShPShJVJmMJSJbVBgTlmbM
GzjpHzGHHfQNfJSlPQbJBSTJBM
HWpspsWjNsGRHfpwNszzLfGqnCnZtdvvcZvwqFCcZqMZCv
rhHhDhhDrRNwDRhNRLRqNPHjHSpVpVlljSSMnlHnjS
sVVBsFBJBtBSPSjtjPPZZS
JmddVTzsffcGNwzCqNqR
nQfqFtZWFZnFJWfpGqhDsGLLPLVbrPhr
CzjwMTgBgTNSGLVVDPbjbS
wMCmgNNdgvzGdRQcFGFdfJ
CbqCDnwFpDpCVfBPmPffPfRfpB
svcsjlGJGnlnvjvzgQQgGzsZPhBRcRPhWfBZBhPhWRRhPN
sJngnjSGGTGzgJGrrbCqLHrCLwSFqV
hnRnJrwMHnQRRRwMhRrnJRBLZtBPdPSdtZZjjstsLHBP
pBBzzcbTbzCcFzTvlSCZLCtSvvttdZ
FWmNzBFNDnRqNRDM
lvzlSPRDSpDJmNwNGgFpfsFN
WBHrtrLBbhWHtdrFwFFsjzwfFjsdsF
HWnrHnTWhWTCTzTBnRZqDDDCqZcclRvSJR
qcdVbpcMFQcdMGcMFjjpbCnCGmCnJHDHDGJChzCJCJ
wWwlSfBRgRNBDPJnCgCrgzms
tBTfTRwlwRLNwTRmWtNwQFZtqcjcdcqpbdQMqMdd
jJRzwDdwFdJddcjjFCFvQLvNlBhSBhCC
gMMpbHpmnHpWfnlCSBQCPmPllQSs
nHWHVfTGdTSjZzDT
sCVTsBPltgDSbLvRMMDT
WNrjNmWZwmZwfVLvLRbRNvMp
mrHGrmnjqVzjGqdmCltlhFFllgsHBPCg
wbglgTFRblnMRltJBNrDDWQbtdbN
dqCpcjPLGfHZQBJNGGvQDZ
SPssjVSqPCLpScfCgFRVmTlRwndlRwTR
hWMWMhTTDgWMzGMszQShGWJPCQmPVCNPffPfVJftqNQC
nbnHHwLrnswwPVCPZV
nsRvFHLsvplvrcsSWDDDddjjDlMTjT
HRCnhWZhCddgwRcwhdWZHHQLJzSLsSzjJwSjztsmtzsmLF
VlVVNNVqTlMVbTVVMqvqvrDTjQfFtjstJmzftfzsSbtStJQb
pQvvPpBqNBccPWWhcWWc
glJTndVWCTDDVFvnVndVlCvwmBmqBBwQmwPwmMPggGsPGm
NhcSrhNZHhZzRtHcNRrSMSMSFBMGmsQQGLMPPF
hcpRzbcHjRhRbZRzZRztzRpJVdCdnfCnlFvdDTnJCVDjWd
dqWvjjBdWWqMjdvvMJjWWjMGgcfchhzrhwbrwShwgzSqNb
mmmmsVTlVlsLQmnpsNcfgfwNLgczhcGhzw
RslpsnGGGlWdCZFMMRWR
pfSpZSrdSMVDVVLMMDDZpdgRWMPGqqPGqmqqGGhGHNRN
wwQBwtjvvJlvsnbTbvBRHgPmlqhlqWWHqZZgNm
JnQwJvjTtzTZwJnbzzfCrfLCfdfdddfD
PZcnljZFTVmQdlQh
JCLLqBBCSNCCqzJNzStBpStBmsdhVTdmVWsWmdhfLhWVdfrR
BpMzzqSzppMMqpJqqgzPFcjZjnTjgcjbPjPDPZ
HLHWmqBHHqWbMHFtbgWcdhspPcPchndPpn
ZRVSQZRfVZGRgnfsdhLgLndL
jzJRwJRSJJvSlZQGRVwlSGZDMqmmCCCmtNbCmtCMzFNLbF
fDhlBhhZmQRRfHwLdjHFFWQQjQ
ZMTssVVzvbCqJddsLwwH
TzgTbgGGzztMbbvzvVbGvSPBrlShRrfnNrnBDlNcRDZRZB
bjfqGfvFfcHvRwGFRHjbgQtddlQljWpWnQgdWWll
TSPVVSVwSzSDTDlntDndnlWddt
hTrrNCBCVrCrrVshThHHbZvFGscwbfFGRJcZ
jNhMjcgvMNgWggvttcFtchvPFFzdPPrQrrGGQQszRPQrRG
wSCqCmmppbTwPnGHrQdrGCCD
LlqwGmGScLcVthVt
dGGrWWDqmCnwCCQMQrMbFHbMHsPFgPVZbgFPgg
tLRTBwfvTBJcZFNFZRHSHPVN
zjJLTvjcwDzqqhGD
DzrWszFFrtBBhnhNCClHlnHbSbmlSn
ZLwLcVVZcLVpvRwLgMLpLJgPmmQSNSTNbtTvQmCHQCClCHCt
cfgZJwwVLJZPPVpRwzhzrrfGGBrGtDrBsf
BTsdCQsQnwwdcCqqdCnsFvGFpFBvGzrLLmmzpvfG
pjZjgPVlJVMVtgJSWLPvfFFFbNNmNbvbmL
ggVplgJtHSSggdCHsscChhdnnR
JddZcSlvvGFJNWVWFgQgVtFWhg
nDqsHqCszwjCCPMnfhfBhtdWggfg
bqppHqqHHbPLjLCppbwDdRNvdmcTmlNZGGbTTcNv
lcZDSvztcHHcMSZVHVvMZBqBNNFNhBNTTmssBqBcFB
bbGJQdQPpGfQJQdJfLFhmznhzLLNTFhNhf
JGpWpPJddbPpPwpjbtZSzltDgWDSgvtrVv
lJcNlNjPcmtFzHtHBJFg
VPZdwGGWVrsdPWhWwhzzCzzFppDFMBtttFCr
wshsLVfWTnGswdhwcmRjRmScqTcjlPNq
BsBshRZQQsVdsZQZSdsPDwJDHNNHBztNNFMGGwMF
nSTpLjcvHppzwwNt
jcqncjjvfmgrCfvqrnZbmdmSsdbVWlsmhRdW
FjjqRZjZFZWFqPvNvvPQpmbPDCmt
GSnHSnrScncHhrtncGshVbmTmbpVvppCmpCTmTCmCC
rHfnddwdfctlzFFgwqlj
wBwlBmmhwRVThVBmFHnvHMnfsmFfHq
jzwGSGJNZCCssHfsCPfv
GJbzbZdbwJdtdQphRVWQ
ztFZccVHFWHHLSDBpSBPhhZw
fmjsCmqnNTJnvbTvLRPDlCpBSPBpRShl
JqsGTSNJGqvqvNvttGQQGdFrQctrrF
mNZqjTFrZqrTNTTGSSSbrhrhRFRHcnLCzcCdHcLBdccHLzRC
JDDfJswJDWsvgVgwpWnRBlBzGCnlCLBlcCDl
vVGpfQWJpsPQMVgvppVwgWPJrqSZTrZqhTQTrhjbjqSSjZqt
fMSDzDHzpDDVsStdDgwwFZFrrMPCNngCZP
WmLnbWmvvWTTLWWQWRGQvLvrPFZJcgJFCZNrCgGCFCPNPF
QLhlqhljWvRQbbqlqnfStnHdBqVVfzpq
qWZtSQTSvJJvBfJVBBVFNDNHbbdRVPdpNFRF
fGcwwmCgsLhgwLchbpHdrrrphdPRPh
gMjllmcjwsLMgcwlMnSWzSqjSZJqZqZfjQWv
cdRHPjRFRdFVHGcFfFTHQTHVLSQBsbsqSCLqllBJqqbSqLJC
WwWppWgtNrnzzWDmrrmNvWJClJSSbqLJJCGbsCgJbLLg
MnvWpmwvWnvtGPjVRcHRRHMj
dLMDhdHGrcLTvvvstB
nPqgGGNPqRgRSjgmlWjbbTTPPBvpvpTcswwsTF
NRGmgjRnWgnZJqllmSqjnqSDhQzVrdQHDfDrDJQDMHDJzM
RgmcPmGNQwwNmSRwPPgfmrBlCDlGbvFFvtrCsvlrBB
nfdVnjHdMWnTqflFtbjrllrlsCDl
TZJVMhnfMnVMHnpRRNRzgJRzSzgwPR
nTbsblzlnGllmsNnbDwbcWQWwWBFJBQcWQvPWFJM
HRZRdSRdCdLdRftrHHZfSQBMWJMgMzQPFWMJCFWgvQ
HVzrLrrjRjLGnlTnlDlsjj
RQdTdZhWqlZhTdWTqblhNmTMVnnrQsnnpvgMVMHMgHMrgP
fjSSfjcCzGNBjCjsnpMVpnpzvpzsHp
GGwLGSccwjwLwBcGLGSqZWmmdLZLWThmRNZmWR
ncmBrmfdfcVcfGnQdVWRBRvgqNvDvWqzLRqg
PlpPbSbLFPLpFstsbqRqZMqWZqMWSDRzvq
sjtjHCwJwlHfdfjccmmLLQ
tGMtLHQGWzLHFVQtVfQtMWtbgrZNbdfSbccggrcTjTrfbm
swRChnwqhBCCCmSjcNSdcd
vnlnqDRlsRnJJqswJvGMFVMMdWtpVJFMQQFL
ffcHLzGmfvqqfWfF
rJrrRSPCPMCrPRQMNNWJQjJCnqFVVVTStqVnSBdqppBVtpqd
jNQQPjQMCRQwwQbRQbjMgwbNhwWZHHmzmlchlHHHDcLmWzhD
JzvrRHHJvCRZFPFnPgsQVVQNzQTQDttVsB
GSMjpqdbGNppBtDhPN
MbmqMwGbZPmPHJmv
ggVSVWzCNbbNCbRM
DfVhVsVQcQDmpmQTTQLjPjMjbDGNlbRMlNGG
sftTFJJccnphcfncTsfBvwZvrzgzZzZvBrVnzq
qqlClBNSCNSRQMvdrwFvnBrr
GDfhDtszhhsThpTDzsfpprMrFvQvrnnfvQrwHFbfFH
VtggWhpsmGVTGJWMjWjNjPWjLL
NSnShnjsswSsRPNsrnwcwMHfFwGqbvqCbFfg
LDJtzzmgVVlvqGbzvFfzff
mlJmQQpmJpZpBJJdjBNhrnnRdTTgNN
RtRRvbhDFPHHlhtPhRvPRtqjmzqzzwLjHQHLLLQVmVjq
WNNBBZfgfWnqVQwNqzmQcm
MZGfzSrTTnWrrWsTWnfSGbGllPCbGlJRvlPllCtt
phgcNfqgfpZsjjpdsS
brHHnWPbDPDbTPlDJJvJJPrMVVzRSdFzwdZZzFnsdzzVdVZs
SbStvJMDQgNmmtgg
lzDMDhfFRlfMFTfMGPMbFTlMpBHrmpjjCFjmBmrqBjtCmjjq
VvVswLLHZnJJwdndNsSSNBpCpjqjmNgrmCNtmmqm
nHcZWcSVsScTDlPPlTPb
DFMQSlMDpSpFDtDFccHvmqzvbHZjJmvzmmQH
CSgffrPssdgqRbqzzRqZ
dsdsfGsrBPsTVcMctMVNNhtStW
MDWRDWpgDvWpNptvNMnJCHJHMwHCndJfZn
blcrqTFmmcbhLTTTmlBZbjBJdBfBnCbQBdfn
TzlnhcnzTmhLRvSDsRzDPzWR
nwmmPnnPDjclhhjfFzzzwqbFFNHwVqVq
vSQrbpWCvMWQQWQMLgFJVHqzBzJVNqzBFL
CvCvQtWWQmcbtntPcb
qzvtzCCtLsLLzmQCHqpSsHSmfrnNrTrNWWMNGnWZTMZGBvWn
PglVFJwPhbcghVTBpBnWWhWnTMhB
FgjcJccVpwDcwFgVDsqzddqLqSDSCQQL
PLHsSVGGPvSLTffjMJWJJBjfBL
gNhwgTqDcqwpDqNhFpDFhFWfnndjcMJjMWzzBtntJBnW
DFhhQRRwbCbwRQChppmTHCVGTlslrsvrrSVZ
hHnRfSMmsSVrFvQqrmDPgr
jjcBzjZLpWccJLczBjZjWGwCnNzDvrgPCgQvFggvqrQFnr
jtZGpLwjWBpBWcfMTttfbHRnHTbT
swSHffFTVrJlHFSWrTpMZMPhMNBqBhTvhhqZ
bQbLQGGjQLdRCcQjGZBzpzBZqMBBBVdBqh
RjQDQGmgmVVbVcjjmjgcnWtJfrwsfrtHlwswnfHSFw
bVHbbMFDcbDbcmbbHVRbMRFgzSmdzSSTBtTjBdQTzlSldQjT
nqpqCMwJffqQSzTBQlqd
wvJhnnfrfsJrCNffvspRrPPFMbPMRRPPMZgZHR
FnJZnssHvMdJWJpW
mlDlllGrSGmttwlGvZgrzZNWgggzTzdT
bCltZCthtRcbcFbjFL
JGmHrJwGzzpllRZdzZLRcW
htbPSbVtFbbgjhffgPSfTVSDcHDDDLWscRdsZjdZcRssWc
vhfFbFvVHbtTVgFPhmJBCvBNrqBBrGCqQB
BcNQcvcBchSQNccLLvhTqbJZTrHrrrzzqTZMZMFZ
tpHDsnDRslllCCHtwnpsfjRgrJrVMfMmMrFrzVJFzrVZbZrr
tnDpjCwPLNSPdhPH
qnjvvBwBhSSFPgDQLVVDqgLr
WbszTbHTbsbHJWHLLMVZmJPQDrvmZL
ctlRbtCWWtvlSfjljjhBnBNB
TsnvssDDQlRbzMzjDMqHwq
tSGjZCfFZtCFLtVGWGZFbcqwHbhWdbzbwHbdwbdw
ZGBFJFZVBLjStZPBBZLRvlmsPTvsgrrnrTllvs
PgQdNsQFsdNwWqQBsQrTrTLpbrnTpGngnbTG
hCzzMJVDmfzmBDMCfSfhTMGcnpnTcjGrpjbMGjbL
CVVfvCflSHNHvPdBHW
DwlMjMNjStgmthMghg
PTlpHnJJTcZvTTbHZWZTvpqdHhmLgrgdfrhLLsmshmsLts
vncqTbPqcpCnbCPvccZbDBCGzzNBwjDlVVGFjjVR
QsdGGCztZVRddPgndf
DrNNBbwNHNwlbjFbbCNjNwDWWDPPVfDMgfmVMfnMWmmP
rvJBbbBNcLCtJZQL
NfLlqLhbNPddLPqLhpgHwFFwFHHTwRHWwPFTrT
MSMSCnjBnBjCscjVDVljTvHmmWnrwTrwFTrvTWTT
JCMMBzDMJcZZCjDzSBDNJgdfdQlqlLNdhgGLhp
"###;

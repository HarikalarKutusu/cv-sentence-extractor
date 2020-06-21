use super::*;
use std::path::PathBuf;

#[test]
fn test_extractor() {
    let texts = load(&PathBuf::from("src/test_data/wiki_00")).unwrap();
    let mut iter = SentenceExtractor::new(texts[0].as_str(), false, 3, 38, vec!['，', '：', '；']);
    assert_eq!(iter.next().unwrap(), "愛因斯坦係一位理論物理學家");
    assert_eq!(
        iter.next().unwrap(),
        "佢最出名嘅係發表咗相對論，另外喺量子力學、統計力學、同埋宇宙學方面都有好大貢獻"
    );
    assert_eq!(
        iter.next().unwrap(),
        "愛因斯坦喺德國烏爾姆市出世，一年後成家人搬咗去慕尼黑"
    );
    assert_eq!(iter.next().unwrap(), "佢屋企都係猶太人，但係冇入猶太教");
    assert_eq!(iter.next().unwrap(), "佢爸爸本來賣床褥，後來開電器舖");
    assert_eq!(
        iter.next().unwrap(),
        "五歲嗰年，佢爸爸送咗個指南針畀佢，佢就發現有啲睇唔到嘅嘢牽引住枝針"
    );
    assert_eq!(
        iter.next().unwrap(),
        "後來愛因斯坦話嗰次嘅經驗係佢一生中最有啟發性"
    );
    assert_eq!(
        iter.next().unwrap(),
        "雖然佢識砌啲機械模型嚟玩，但係讀書讀得好慢"
    );
    assert_eq!(
        iter.next().unwrap(),
        "可能係因為學習障礙病，又或者只係因為怕醜，又或者係因為佢個腦結構特殊"
    );
    assert_eq!(
        iter.next().unwrap(),
        "最新嘅理論話愛因斯坦應該係患咗亞氏保加症"
    );
    assert_eq!(iter.next().unwrap(), "係自閉症嘅一種");
    assert_eq!(
        iter.next().unwrap(),
        "因為當時呢個病未畀人發現，佢父母重以為佢係低能"
    );
    assert_eq!(
        iter.next().unwrap(),
        "愛因斯坦話自己之所以諗得出相對論，正係因為細個時學嘢慢，遲過其他小朋友開始思索時空"
    );
    assert_eq!(iter.next().unwrap(), "到嗰陣思想已經比較成熟");
    assert_eq!(iter.next().unwrap(), "後來佢又寫咗好多有關時空，物質嘅理論");
    assert_eq!(
        iter.next().unwrap(),
        "不過，因為當時嘅人睇唔明佢嘅理論，導致佢無法得到應有嘅尊重"
    );
    assert_eq!(iter.next().unwrap(), "但至今，重有好多都睇唔明佢寫乜");
    assert_eq!(
        iter.next().unwrap(),
        "不過，最大唔同嘅係，人已經尊重佢，而唔係當佢癲佬"
    );
    assert!(iter.next().is_none());
}

#[test]
fn test_extractor_with_bondary_condition() {
    let texts = load(&PathBuf::from("src/test_data/wiki_01")).unwrap();
    let mut iter = SentenceExtractor::new(texts[0].as_str(), false, 1, 1, vec!['，', '：', '；']);
    assert_eq!(iter.next().unwrap(), "春");
    assert_eq!(iter.next().unwrap(), "花秋");
    assert_eq!(iter.next().unwrap(), "月何時");
    assert_eq!(iter.next().unwrap(), "了往事知");
    assert_eq!(iter.next().unwrap(), "多");
    assert_eq!(iter.next().unwrap(), "少小");
    assert_eq!(iter.next().unwrap(), "樓昨夜");
    assert_eq!(iter.next().unwrap(), "又東風故");
    assert_eq!(iter.next().unwrap(), "國、不堪、回首月、明中雕欄");
    assert_eq!(iter.next().unwrap(), "玉");
    assert_eq!(iter.next().unwrap(), "砌");
    assert_eq!(iter.next().unwrap(), "應在「只」是《朱》顏");
    assert_eq!(iter.next().unwrap(), "改『問』君【能】有…幾—多．愁");
    assert!(iter.next().is_none());
}
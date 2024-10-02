use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::PartialEq;

macro_rules! serde_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Deserialize, Serialize, Debug, Clone)]
        #[serde(rename_all = "PascalCase")]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PubmedArticleSet {
    pub(crate) pubmed_article: Vec<PubmedArticle>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PubmedArticle {
    pub(crate) medline_citation: MedlineCitation,
    pub(crate) pubmed_data: PubMedData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PubMedData {
    pub(crate) article_id_list: ArticleIdList,
    pub(crate) reference_list: Option<ReferenceList>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Reference {
    #[serde(deserialize_with = "unwrap_string")]
    pub(crate) citation: String,
    pub(crate) article_id_list: ArticleIdList,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ReferenceList {
    pub(crate) reference: Vec<Reference>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MedlineCitation {
    #[serde(rename = "PMID")]
    pub(crate) pmid: PMID,
    pub(crate) date_completed: Date,
    pub(crate) date_revised: Date,
    pub(crate) article: Article,
    pub(crate) medline_journal_info: MedlineJournalInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MedlineJournalInfo {
    #[serde(deserialize_with = "unwrap_string", rename = "NlmUniqueID")]
    pub(crate) id: String,
    #[serde(deserialize_with = "unwrap_string")]
    pub(crate) country: String,
    #[serde(deserialize_with = "unwrap_string", rename = "ISSNLinking", default)]
    pub(crate) issn: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct JournalIssue {
    pub(crate) volume: Option<String>,
    pub(crate) issue: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Journal {
    #[serde(rename = "ISSN")]
    pub(crate) issn: Option<ISSN>,
    pub(crate) journal_issue: Option<JournalIssue>,
    pub(crate) title: String,
    #[serde(rename = "ISOAbbreviation")]
    pub(crate) iso_abbreviation: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Article {
    #[serde(rename = "@PubModel")]
    pub(crate) pub_model: String,
    pub(crate) journal: Journal,
    pub(crate) article_title: String,
    pub(crate) author_list: Option<AuthorList>,
    pub(crate) publication_type_list: PublicationTypeList,
    pub(crate) grant_list: Option<GrantList>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Grant {
    #[serde(rename = "GrantID")]
    pub(crate) grant_id: String,
    pub(crate) acronym: Option<String>,
    pub(crate) agency: String,
    pub(crate) country: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct GrantList {
    pub(crate) grant: Vec<Grant>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ArticleId {
    #[serde(rename = "@IdType")]
    pub(crate) ty: String,
    #[serde(rename = "$value")]
    pub(crate) id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ArticleIdList {
    pub(crate) article_id: Vec<ArticleId>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct PublicationType {
    #[serde(rename = "@UI")]
    pub(crate) id: String,
    #[serde(rename = "$value")]
    pub(crate) name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PublicationTypeList {
    pub(crate) publication_type: Vec<PublicationType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Author {
    #[serde(deserialize_with = "unwrap_string", default)]
    pub(crate) last_name: String,
    #[serde(deserialize_with = "unwrap_string", default)]
    pub(crate) fore_name: String,
    #[serde(deserialize_with = "unwrap_string", default)]
    pub(crate) initials: String,
    #[serde(deserialize_with = "unwrap_string", default)]
    pub(crate) collective_name: String,
}

fn unwrap_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub(crate) struct OptionStringUnwrapper {
        #[serde(rename = "$value")]
        field: String,
    }
    Ok(OptionStringUnwrapper::deserialize(deserializer)?.field)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct AuthorList {
    #[serde(rename = "Author")]
    pub(crate) list: Vec<Author>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ISSN {
    #[serde(rename = "$value")]
    pub(crate) id: String,
    #[serde(rename = "@IssnType")]
    pub(crate) ty: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PMID {
    #[serde(rename = "$value")]
    pub(crate) id: u64,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Date {
    year: u16,
    month: u8,
    day: u8,
}

pub(crate) fn serde(content: &str) {
    let res = quick_xml::de::from_str::<PubmedArticleSet>(content).unwrap().pubmed_article;
    res.iter().filter(|e| !e.pubmed_data.reference_list.is_none())
        .take(10)
        .for_each(|e| println!("{:#?}", e))
    // for article in &res.pubmed_article {
    //     if article.medline_citation.article.author_list.is_none() {
    //         // println!("{:?}", article);
    //         continue;
    //     }
    //     for author in article.medline_citation.article.author_list.clone().unwrap().list {
    //         if author.fore_name == "" || author.initials == "" {
    //             println!("{:?}", article);
    //         }
    //     }
    // }
}
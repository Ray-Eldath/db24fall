use crate::de::EnumAuthor::{Collective, Person};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::PartialEq;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct PubmedArticleSet {
    pub(crate) pubmed_article: Vec<PubmedArticle>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct PubmedArticle {
    pub(crate) medline_citation: MedlineCitation,
    pub(crate) pubmed_data: PubMedData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct PubMedData {
    pub(crate) article_id_list: ArticleIdList,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) reference_list: Vec<ReferenceList>,
}

fn pubmed_reference_list_deser<'de, D>(deserializer: D) -> Result<Option<ReferenceList>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    struct Tmp {
        #[serde(default)]
        reference_list: Vec<ReferenceList>,
    }
    let tmp: Tmp = Deserialize::deserialize(deserializer)?;
    if tmp.reference_list.is_empty() {
        Ok(None)
    } else {
        Ok(Some(tmp.reference_list[0].clone()))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct Reference {
    #[serde(deserialize_with = "join_segmented_string")]
    pub(crate) citation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) article_id_list: Option<ArticleIdList>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct ReferenceList {
    // TODO some ReferenceList may contains another ReferenceList. here we just ignore them.
    #[serde(default)]
    pub(crate) reference: Vec<Reference>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct MedlineCitation {
    #[serde(rename(deserialize = "PMID"))]
    pub(crate) id: PMID,
    pub(crate) date_revised: Date,
    pub(crate) date_completed: Option<Date>,
    pub(crate) article: Article,
    pub(crate) medline_journal_info: MedlineJournalInfo,
    #[serde(default)]
    pub(crate) keyword_list: Vec<KeywordList>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct MedlineJournalInfo {
    #[serde(
        deserialize_with = "unwrap_string",
        rename(deserialize = "NlmUniqueID")
    )]
    pub(crate) id: String,
    #[serde(deserialize_with = "unwrap_string")]
    pub(crate) country: String,
    #[serde(
        deserialize_with = "unwrap_string",
        rename(deserialize = "ISSNLinking"),
        default
    )]
    pub(crate) issn: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct JournalIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) issue: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct Journal {
    #[serde(rename(deserialize = "ISSN"))]
    pub(crate) issn: Option<ISSN>,
    pub(crate) journal_issue: Option<JournalIssue>,
    pub(crate) title: String,
    // #[serde(rename(deserialize = "ISOAbbreviation"))]
    // pub(crate) iso_abbreviation: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct Article {
    #[serde(rename(deserialize = "@PubModel"))]
    pub(crate) pub_model: String,
    pub(crate) journal: Journal,
    /// article_title may contain \<sub\> or other HTML tags
    #[serde(deserialize_with = "join_segmented_string")]
    pub(crate) article_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) author_list: Option<AuthorList>,
    pub(crate) publication_type_list: PublicationTypeList,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grant_list: Option<GrantList>,
}

fn raw_de<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let str: Result<String, _> = Deserialize::deserialize(deserializer);
    str.or(Ok("Corrupted Article Title".to_string()))
}

fn join_segmented_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "snake_case"))]
    enum TextOrOther {
        Text(String),
        B(String),
        Other,
    }
    #[derive(Deserialize, Debug)]
    struct TextOrOtherWrapper {
        #[serde(rename(deserialize = "$value"), default)]
        field: Vec<ItalicBoldString>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "snake_case"))]
    enum ItalicBoldString {
        Sup,
        Sub,
        I(String),
        B(ItalicBoldStringWrapper),
        #[serde(rename = "$text")]
        String(String),
    }
    #[derive(Deserialize, Debug)]
    struct ItalicBoldStringWrapper {
        #[serde(rename(deserialize = "$value"), default)]
        field: Vec<ItalicBoldString>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "snake_case"))]
    enum CouldBeString {
        Sup(ItalicBoldStringWrapper),
        Sub(ItalicBoldStringWrapper),
        I(ItalicBoldStringWrapper),
        B(ItalicBoldStringWrapper),
        Math,
        #[serde(rename = "$text")]
        String(String),
    }
    #[derive(Deserialize, Debug)]
    struct SegmentedString {
        #[serde(rename(deserialize = "$value"), default)]
        field: Vec<CouldBeString>,
    }

    fn traverse_ibs_wrapper(ibs: &ItalicBoldStringWrapper) -> String {
        ibs.field
            .iter()
            .map(|e| match &e {
                ItalicBoldString::I(str) => str.clone(),
                ItalicBoldString::B(str) => format!("{:?}", str),
                ItalicBoldString::String(str) => str.clone(),
                _ => "".to_string(),
            })
            .collect()
    }

    // Ok(SegmentedString::deserialize(deserializer)?.field.join(" "))
    Ok(SegmentedString::deserialize(deserializer)?
        .field
        .iter()
        .map(|e| match e {
            CouldBeString::I(str) => traverse_ibs_wrapper(&str),
            CouldBeString::B(str) => traverse_ibs_wrapper(&str),
            CouldBeString::Sup(str) => traverse_ibs_wrapper(&str),
            CouldBeString::Sub(str) => traverse_ibs_wrapper(&str),
            CouldBeString::String(str) => str.clone(),
            &CouldBeString::Math => "".to_string(),
        })
        .map(|e| e.trim().to_string())
        .collect::<Vec<_>>()
        .join(" "))
}

mod tests {
    use super::*;

    #[test]
    fn join_segmented_string_test() {
        let xml = r"
        <article>
            <title>
            text <sub>1-<i>y</i></sub>
            </title>
        </article>
        ";
        #[derive(Deserialize, Debug)]
        struct ABA {
            #[serde(deserialize_with = "join_segmented_string")]
            title: String,
        }
        #[derive(Deserialize, Debug)]
        struct AnyName {
            // Does not (yet?) supported by the serde
            // https://github.com/serde-rs/serde/issues/1905
            // #[serde(flatten)]
            #[serde(deserialize_with = "join_segmented_string")]
            title: String,
        }

        let xd = &mut quick_xml::de::Deserializer::from_str(xml);
        let res: Result<AnyName, _> = serde_path_to_error::deserialize(xd);
        println!("{:#?}", res.unwrap());
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct Grant {
    #[serde(rename(deserialize = "GrantID"))]
    pub(crate) id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) acronym: Option<String>,
    pub(crate) agency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) country: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct GrantList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) grant: Vec<Grant>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
#[serde(transparent)]
pub(crate) struct Keyword {
    #[serde(deserialize_with = "join_segmented_string")]
    pub(crate) name: String,
    // TODO: is major?
    // #[serde(rename(deserialize = "@MajorTopicYN"), deserialize_with = "unwrap_yn")]
    // pub(crate) is_major: bool,
}

fn unwrap_yn<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)? == "Y")
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct KeywordList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) keyword: Vec<Keyword>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct ArticleId {
    #[serde(rename(deserialize = "@IdType"))]
    pub(crate) ty: String,
    #[serde(rename(deserialize = "$value"))]
    pub(crate) id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct ArticleIdList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) article_id: Vec<ArticleId>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct PublicationType {
    #[serde(rename(deserialize = "@UI"))]
    pub(crate) id: String,
    #[serde(rename(deserialize = "$value"))]
    pub(crate) name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct PublicationTypeList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) publication_type: Vec<PublicationType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
#[serde(untagged)]
pub(crate) enum EnumAuthor {
    Person {
        last_name: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        fore_name: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        initials: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        affiliation: Vec<String>,
    },
    Collective {
        collective_name: String,
    },
}

#[derive(Deserialize)]
pub(crate) struct StringValueUnwrapper {
    #[serde(rename(deserialize = "$value"))]
    field: String,
}

fn de_vec_enum_author<'de, D>(deserializer: D) -> Result<Vec<EnumAuthor>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    struct FlatAuthorAffiliation {
        #[serde(deserialize_with = "unwrap_string", default)]
        affiliation: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    struct FlatAuthor {
        #[serde(deserialize_with = "unwrap_string", default)]
        last_name: String,
        #[serde(deserialize_with = "unwrap_string", default)]
        fore_name: String,
        #[serde(deserialize_with = "unwrap_string", default)]
        initials: String,
        #[serde(deserialize_with = "unwrap_string", default)]
        collective_name: String,
        #[serde(default)]
        affiliation_info: Vec<FlatAuthorAffiliation>,
    }
    let lst: Vec<FlatAuthor> = Deserialize::deserialize(deserializer)?;
    lst.iter()
        .map(|de| {
            let person_empty =
                de.last_name.is_empty() && de.fore_name.is_empty() && de.initials.is_empty();
            let collective_empty = de.collective_name.is_empty();
            if person_empty ^ collective_empty {
                if collective_empty {
                    Ok(Person {
                        last_name: de.last_name.clone(),
                        fore_name: de.fore_name.clone(),
                        initials: de.initials.clone(),
                        affiliation: de
                            .affiliation_info
                            .iter()
                            .map(|de| de.affiliation.clone())
                            .collect(),
                    })
                } else {
                    Ok(Collective {
                        collective_name: de.collective_name.clone(),
                    })
                }
            } else {
                Err(Error::custom(format!(
                    "person_empty={} collective_empty={}",
                    person_empty, collective_empty
                )))
            }
        })
        .collect()
}

fn unwrap_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(StringValueUnwrapper::deserialize(deserializer)?.field)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct AuthorList {
    #[serde(
        rename(deserialize = "Author"),
        deserialize_with = "de_vec_enum_author"
    )]
    pub(crate) author: Vec<EnumAuthor>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct ISSN {
    #[serde(rename(deserialize = "$value"))]
    pub(crate) id: String,
    #[serde(rename(deserialize = "@IssnType"))]
    pub(crate) ty: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct PMID {
    #[serde(rename(deserialize = "$value"))]
    pub(crate) id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "snake_case"))]
pub(crate) struct Date {
    year: u16,
    month: u8,
    day: u8,
}

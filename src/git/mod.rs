
pub mod deprecated;

use chrono::{TimeZone, Utc, DateTime, FixedOffset, Duration};

use git2;
use git2::{Repository, SORT_TIME};

#[derive(Debug)]
pub struct Signature {
    pub name: Option<String>,
    pub email: Option<String>,
    pub when: DateTime<FixedOffset>,
}

impl<'a> From<git2::Signature<'a>> for Signature {
    fn from(sig: git2::Signature) -> Self {
        let time = sig.when();
        Signature {
            name: sig.name().map(|x| x.to_owned()),
            email: sig.email().map(|x| x.to_owned()),
            when: to_datetime(time.seconds(), time.offset_minutes()),
        }
    }
}

#[derive(Debug)]
pub struct Commit {
    pub rev: String,
    pub commit_datetime: DateTime<FixedOffset>,
    pub message: Option<String>,
    pub author: Signature,
    pub committer: Signature,
}

impl<'a> From<git2::Commit<'a>> for Commit {
    fn from(commit: git2::Commit) -> Self {
        let time = commit.time();
        Commit {
            rev: format!("{}", commit.id()),
            commit_datetime: to_datetime(time.seconds(), time.offset_minutes()),
            message: commit.message().map(|x| x.to_owned()),
            author: Signature::from(commit.author()),
            committer: Signature::from(commit.committer()),
        }
    }
}

pub fn get_commits(since: Duration) -> Vec<Commit> {
    let path = ".";
    let repo = Repository::open(path).expect("Repository open");
    let head = repo.head().expect("head").peel_to_commit().expect("peel to commit");

    let head_time: DateTime<Utc> = GitTime::into(GitTime(head.time()));
    let in_since = head_time - since;

    let mut revwalk = repo.revwalk().expect("revwalk");
    revwalk.push_head().expect("push head");
    revwalk.set_sorting(SORT_TIME);

    println!("before revwalk 2");
    let commits: Vec<_> = revwalk.map(|rev| {
            let rev = rev.expect("get rev");
            let commit = repo.find_commit(rev).expect(&format!("couldn't find commit {}", rev));
            Commit::from(commit)
        })
        .take_while(|commit| {
            let commit_date = commit.commit_datetime.with_timezone(&Utc {});
            commit_date > in_since
        })
        .collect();
    commits
}

fn to_datetime(epoch_seconds: i64, timezone_offset_minutes: i32) -> DateTime<FixedOffset> {
    let minute = 60;
    let datetime = Utc.timestamp(epoch_seconds, 0);

    datetime.with_timezone(&FixedOffset::east(timezone_offset_minutes * minute))
}

fn to_datetime_utc(epoch_seconds: i64) -> DateTime<Utc> {
    Utc.timestamp(epoch_seconds, 0)
}

struct GitTime(pub git2::Time);

impl From<GitTime> for DateTime<Utc> {
    fn from(time: GitTime) -> Self {
        let GitTime(time) = time;
        Utc.timestamp(time.seconds(), 0)
    }
}

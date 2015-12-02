//! Rust representations of Github API data structures

use std::collections::HashMap;
use std::hash::Hash;
use rustc_serialize::json::{Json, ToJson};
use rustc_serialize::{Decoder, Decodable, Encodable, Encoder};
use statuses::State;

#[derive(Debug, RustcDecodable)]
pub struct FieldErr {
    pub resource: String,
    pub field: String,
    pub code: String
}

#[derive(Debug, RustcDecodable)]
pub struct ClientError {
    pub message: String,
    pub errors: Option<Vec<FieldErr>>
}

impl Decodable for Deployment {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Deployment, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      Ok(Deployment {
        url: try!(decoder.read_struct_field("url", 0, |decoder| Decodable::decode(decoder))),
        id: try!(decoder.read_struct_field("id", 0, |decoder| Decodable::decode(decoder))),
        sha: try!(decoder.read_struct_field("sha", 0, |decoder| Decodable::decode(decoder))),
        commit_ref: try!(decoder.read_struct_field("ref", 0, |decoder| Decodable::decode(decoder))),
        task: try!(decoder.read_struct_field("task", 0, |decoder| Decodable::decode(decoder))),
        environment: try!(decoder.read_struct_field("environment", 0, |decoder| Decodable::decode(decoder))),
        description: try!(decoder.read_struct_field("description", 0, |decoder| Decodable::decode(decoder))),
        creator: try!(decoder.read_struct_field("creator", 0, |decoder| Decodable::decode(decoder))),
        created_at: try!(decoder.read_struct_field("created_at", 0, |decoder| Decodable::decode(decoder))),
        updated_at: try!(decoder.read_struct_field("updated_at", 0, |decoder| Decodable::decode(decoder))),
        statuses_url: try!(decoder.read_struct_field("statuses_url", 0, |decoder| Decodable::decode(decoder))),
        repository_url: try!(decoder.read_struct_field("repository_url", 0, |decoder| Decodable::decode(decoder))),
      })
    })
  }
}

#[derive(Debug)]
pub struct Deployment {
  pub url: String,
  pub id: u64,
  pub sha: String,
  pub commit_ref: String,
  pub task: String,
//  payload: Json,
  pub environment: String,
  pub description: String,
  pub creator: User,
  pub created_at: String,
  pub updated_at: String,
  pub statuses_url: String,
  pub repository_url: String
}

impl Encodable for DeploymentReq {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      DeploymentReq {
        commit_ref: ref cref,
        task: ref tsk,
        auto_merge: ref amrg,
        required_contexts: ref reqctx,
        payload: ref pld,
        environment: ref env,
        description: ref desc
      } => {
          encoder.emit_struct("DeploymentReq", 1usize, |encoder| {
              let mut index = 0;
              try!(encoder.emit_struct_field("ref", index, |encoder| cref.encode(encoder)));
              if tsk.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("task", index, |encoder| tsk.encode(encoder)));
              }
              if amrg.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("auto_merge", index, |encoder| amrg.encode(encoder)));
              }
              if reqctx.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("required_contexts", index, |encoder| reqctx.encode(encoder)));
              }
              if pld.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("payload", index, |encoder| pld.encode(encoder)));
              }
              if env.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("environment", index, |encoder| env.encode(encoder)));
              }
              if desc.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("description", index, |encoder| desc.encode(encoder)));
              }
              Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct DeploymentReq {
  pub commit_ref: String,
  pub task: Option<String>,
  pub auto_merge: Option<bool>,
  pub required_contexts: Option<Vec<String>>,
  /// contents of payload should be valid JSON
  pub payload: Option<String>,
  pub environment: Option<String>,
  pub description: Option<String>
}

impl DeploymentReq {
  pub fn builder<C>(commit: C) -> DeploymentReqBuilder where C: Into<String> {
    DeploymentReqBuilder::new(commit)
  }
}

#[derive(Default)]
pub struct DeploymentReqBuilder {
  pub commit_ref: String,
  pub task: Option<String>,
  pub auto_merge: Option<bool>,
  pub required_contexts: Option<Vec<String>>,
  pub payload: Option<Json>,
  pub environment: Option<String>,
  pub description: Option<String>
}

impl DeploymentReqBuilder {
  pub fn new<C>(commit: C) -> DeploymentReqBuilder where C: Into<String> {
    DeploymentReqBuilder {
      commit_ref: commit.into(),
      ..Default::default()
    }
  }

  pub fn task<T>(&mut self, task: T) -> &mut DeploymentReqBuilder where T: Into<String> {
    self.task = Some(task.into());
    self
  }

  pub fn auto_merge(&mut self, auto_merge: bool) -> &mut DeploymentReqBuilder {
    self.auto_merge = Some(auto_merge);
    self
  }

  pub fn required_contexts<C>(&mut self, ctxs: Vec<C>) -> &mut DeploymentReqBuilder where C: Into<String> {
    self.required_contexts = Some(ctxs.into_iter().map(|c|c.into()).collect::<Vec<String>>());
    self
  }

  pub fn payload<T: ToJson>(&mut self, pl: T) -> &mut DeploymentReqBuilder {
    self.payload = Some(pl.to_json());
    self
  }

  pub fn environment<E>(&mut self, env: E) -> &mut DeploymentReqBuilder where E: Into<String> {
    self.environment = Some(env.into());
    self
  }

  pub fn description<D>(&mut self, desc: D) -> &mut DeploymentReqBuilder where D: Into<String> {
    self.description = Some(desc.into());
    self
  }

  pub fn build(&self) -> DeploymentReq {
    DeploymentReq {
      commit_ref: self.commit_ref.clone(),
      task: self.task.clone(),
      auto_merge: self.auto_merge,
      required_contexts: self.required_contexts.clone(),
      payload: self.payload.clone().map(|p| p.to_string()),
      environment: self.environment.clone(),
      description: self.description.clone()
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct GistFile {
  pub size: u64,
  pub raw_url: String,
  // type: String
//  pub truncated: bool,
  pub language: Option<String>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Gist {
  pub url: String,
  pub forks_url: String,
  pub commits_url: String,
  pub id: String,
  pub description: Option<String>,
  pub public: bool,
  pub owner: User,
  pub user: Option<User>,
  pub files: HashMap<String, GistFile>,
  pub comments: u64,
  pub comments_url: String,
  pub html_url: String,
  pub git_pull_url: String,
  pub git_push_url: String,
  pub created_at: String,
  pub updated_at: String
}

#[derive(Debug, RustcDecodable)]
pub struct GistFork {
  user: User,
  url: String,
  id: String,
  created_at: String,
  updated_at: String
}

impl Encodable for Content {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      Content {
        filename: ref this_filename,
        content: ref this_content,
      } => {
          encoder.emit_struct("Content", 1_usize, |encoder| {
              if this_filename.is_some() {
                  try!(encoder.emit_struct_field("filename", 0_usize, |encoder| this_filename.encode(encoder)));
              }
              try!(encoder.emit_struct_field("content", 0_usize, |encoder| this_content.encode(encoder)));
              Ok(())
          })
      }
    }
  }
}

#[derive(Debug)]
pub struct Content {
  pub filename: Option<String>,
  pub content: String
}

impl Content {
  pub fn new<F, C>(filename: Option<F>, content: C) -> Content where F: Into<String>, C: Into<String> {
    Content { filename: filename.map(|f| f.into()), content: content.into() }
  }
}

impl Encodable for GistReq {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      GistReq {
        description: ref this_description,
        public: ref this_public,
        files: ref this_files
      } => {
          encoder.emit_struct("GistReq", 1, |encoder| {
              let mut index: isize = -1;
              if this_description.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("description", index as usize, |encoder| this_description.encode(encoder)));
              }
              if this_public.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("public", index as usize, |encoder| this_public.encode(encoder)));
              }
              index += 1;
              try!(encoder.emit_struct_field("files", index as usize, |encoder| this_files.encode(encoder)));
              Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct GistReq {
  pub description: Option<String>,
  pub public: Option<bool>,
  pub files: HashMap<String, Content>
}

impl GistReq {
  pub fn new<D,K,V>(desc: Option<D>, public: bool, files: HashMap<K, V>) -> GistReq where D: Into<String>, K: Hash + Eq + Into<String>, V: Into<String> {
    let mut contents = HashMap::new();
    for (k,v) in files.into_iter() {
      contents.insert(k.into(), Content::new(None as Option<String>, v.into()));
    }
    GistReq {
      description: desc.map(|d|d.into()),
      public: Some(public),
      files: contents
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Permissions {
  pub admin: bool,
  pub push: bool,
  pub pull: bool
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Repo {
  pub id: u64,
  pub owner: User,
  pub name: String,
  pub full_name: String,
  pub description: String,
  // private (keyword)
  pub fork: bool,
  pub url: String,
  pub html_url: String,
  pub archive_url: String,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub downloads_url: String,
  pub events_url: String,
  pub forks_url: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  pub hooks_url: String,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  pub languages_url: String,
  pub merges_url: String,
  pub milestones_url: String,
  pub mirror_url: Option<String>,
  pub notifications_url: String,
  pub pulls_url: String,
  pub releases_url: String,
  pub ssh_url: String,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  pub trees_url: String,
  pub homepage: Option<String>,
  pub language: Option<String>,
  pub forks_count: u64,
  pub stargazers_count: u64,
  pub watchers_count: u64,
  pub size: u64,
  pub default_branch: String,
  pub open_issues_count: u64,
  pub has_issues: bool,
  pub has_wiki: bool,
  pub has_pages: bool,
  pub has_downloads: bool,
  pub pushed_at: String,
  pub created_at: String,
  pub updated_at: String,
//  permissions: Permissions
}

#[derive(Debug, RustcDecodable)]
pub struct RepoDetails {
  pub id: u64,
  pub owner: User,
  pub name: String,
  pub full_name: String,
  // todo
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct User {
  pub login: String,
  pub id: u64,
  pub avatar_url: String,
  pub gravatar_id: String,
  pub url: String,
  pub html_url: String,
  pub followers_url: String,
  following_url: String,
  gists_url: String,
  starred_url: String,
  subscriptions_url: String,
  organizations_url: String,
  repos_url: String,
  events_url: String,
  received_events_url: String,
  // type (keyword)
  site_admin: bool
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Commit {
  pub label: String,
  // ref (keyword)
  pub sha: String,
  pub user: User,
  pub repo: Option<Repo>
}

#[derive(Debug, RustcEncodable)]
pub struct LabelReq {
  pub name: String,
  pub color: String
}

impl LabelReq {
  pub fn new<N,C>(name: N, color: C) -> LabelReq where N: Into<String>, C: Into<String>{
    LabelReq {
      name: name.into(),
      color: color.into()
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Label {
  pub url: String,
  pub name: String,
  pub color: String
}

#[derive(Default)]
pub struct PullEditBuilder {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<String>
}

impl PullEditBuilder {
    pub fn new() -> PullEditBuilder {
        PullEditBuilder {
            ..Default::default()
        }
    }

    pub fn title<T>(&mut self, title: T) -> &mut PullEditBuilder where T: Into<String> {
        self.title = Some(title.into());
        self
    }

  pub fn body<B>(&mut self, body: B) -> &mut PullEditBuilder where B: Into<String>{
    self.body = Some(body.into());
    self
  }

  pub fn state<S>(&mut self, state: S) -> &mut PullEditBuilder where S: Into<String> {
      self.state = Some(state.into());
      self
  }

  pub fn build(&self) -> PullEdit {
    PullEdit {
        title: self.title.clone(),
        body: self.body.clone(),
        state: self.state.clone()
    }
  }
}

impl Encodable for PullEdit {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      PullEdit {
        title: ref this_title,
        body: ref this_body,
        state: ref this_state
      } => {
          encoder.emit_struct("PullEdit", 1usize, |encoder| {
              let mut index: isize = -1;
              if this_title.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("title", index as usize, |encoder| this_title.encode(encoder)));
              }
              if this_body.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("body", index as usize, |encoder| this_body.encode(encoder)));
              }
              if this_state.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("state", index as usize, |encoder| this_state.encode(encoder)));
              }
              Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct PullEdit {
  title: Option<String>,
  body: Option<String>,
  state: Option<String>
}

impl PullEdit {
  // todo represent state as enum
  pub fn new<T,B,S>(title: Option<T>, body: Option<B>, state: Option<S>) -> PullEdit where T: Into<String>, B: Into<String>, S: Into<String> {
    PullEdit { title: title.map(|t|t.into()), body: body.map(|b|b.into()), state: state.map(|s|s.into()) }
  }
    pub fn builder() -> PullEditBuilder {
        PullEditBuilder::new()
    }
}

#[derive(Debug, RustcEncodable)]
pub struct PullReq {
  pub title: String,
  pub head: String,
  pub base: String,
  pub body: Option<String>
}

impl PullReq {
  pub fn new<T,H,BS,B>(title: T, head: H, base: BS, body: Option<B>) -> PullReq where T: Into<String>, H: Into<String>, BS: Into<String>, B: Into<String> {
    PullReq {
      title: title.into(),
      head: head.into(),
      base: base.into(),
      body: body.map(|b| b.into())
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Pull {
  pub id: u64,
  pub url: String,
  pub html_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub issue_url: String,
  pub commits_url: String,
  pub review_comments_url: String,
  pub review_comment_url: String,
  pub comments_url: String,
  pub statuses_url: String,
  pub number: u64,
  pub state: String,
  pub title: String,
  pub body: String,
  pub created_at: String,
  pub updated_at: String,
  pub closed_at: Option<String>,
  pub merged_at: Option<String>,
  //pub head: Commit,
//  pub base: Commit,
  // links
  pub user: User,
  pub merge_commit_sha: Option<String>,
  pub mergeable: Option<bool>,
  pub merged_by: Option<User>,
  pub comments: Option<u64>,
  pub commits: Option<u64>,
  pub additions: Option<u64>,
  pub deletions: Option<u64>,
  pub changed_files: Option<u64>
}

#[derive(Debug, RustcEncodable)]
pub struct IssueReq {
  pub title: String,
  pub body: Option<String>,
  pub assignee: Option<String>,
  pub milestone: Option<u64>,
  pub labels: Vec<String>
}

impl IssueReq {
  pub fn new<T,B,A,L>(title: T, body: Option<B>, assignee: Option<A>,
             milestone: Option<u64>, labels: Vec<L>) -> IssueReq where T: Into<String>, B: Into<String>, A: Into<String>, L: Into<String> {
    IssueReq {
      title: title.into(),
      body: body.map(|b|b.into()),
      assignee: assignee.map(|a|a.into()),
      milestone: milestone,
      labels: labels.into_iter().map(|l|l.into()).collect::<Vec<String>>()
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Issue {
  pub id: u64,
  pub url: String,
  pub labels_url: String,
  pub comments_url: String,
  pub events_url: String,
  pub html_url: String,
  pub number: u64,
  pub state: String,
  pub title: String,
  pub body: String,
  pub user: User,
  pub labels: Vec<Label>,
  pub assignee: Option<User>,
  pub locked: bool,
  pub comments: u64,
  pub closed_at: Option<String>,
  pub created_at: String,
  pub updated_at: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Asset {
  pub url: String,
  pub browser_download_url: String,
  pub id: u64,
  pub name: String,
  pub label: Option<String>,
  pub state: String,
  pub content_type: String,
  pub size: u64,
  pub download_count: u64,
  pub created_at: String,
  pub updated_at: String,
  pub uploader: User
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Release {
  pub url: String,
  pub html_url: String,
  pub assets_url: String,
  pub upload_url: String,
  pub tarball_url: String,
  pub zipball_url: String,
  pub id: u64,
  pub tag_name: String,
  pub target_commitish: String,
  pub name: String,
  pub body: String,
  pub draft: bool,
  pub prerelease: bool,
  pub created_at: String,
  pub published_at: String,
  pub author: User,
  pub assets: Vec<Asset>
}

#[derive(Debug, RustcEncodable)]
pub struct ReleaseReq {
  pub tag_name: &'static str,
  pub target_commitish: Option<&'static str>,
  pub name: Option<&'static str>,
  pub body: Option<&'static str>,
  pub draft: Option<bool>,
  pub prerelease: Option<bool>
}


/// builder interface for ReleaseReq
#[derive(Default)]
pub struct ReleaseBuilder {
  tag: &'static str,
  commitish: Option<&'static str>,
  name: Option<&'static str>,
  body: Option<&'static str>,
  draft: Option<bool>,
  prerelease: Option<bool>
}

impl ReleaseBuilder {
  pub fn new(tag: &'static str) -> ReleaseBuilder {
    ReleaseBuilder {
      tag: tag,
      ..Default::default()
    }
  }

  pub fn commitish(&mut self, commit: &'static str) -> &mut ReleaseBuilder {
    self.commitish = Some(commit);
    self
  }

  pub fn name(&mut self, name: &'static str) -> &mut ReleaseBuilder {
    self.name = Some(name);
    self
  }

  pub fn body(&mut self, body: &'static str) -> &mut ReleaseBuilder {
    self.body = Some(body);
    self
  }

  pub fn draft(&mut self, draft: bool) -> &mut ReleaseBuilder {
    self.draft = Some(draft);
    self
  }

  pub fn prerelease(&mut self, pre: bool) -> &mut ReleaseBuilder {
    self.prerelease = Some(pre);
    self
  }

  pub fn build(&self) -> ReleaseReq {
    ReleaseReq::new(self.tag, self.commitish, self.name, self.body, self.draft, self.prerelease)
  }
}

impl ReleaseReq {
  pub fn new(tag: &'static str, commit: Option<&'static str>, name: Option<&'static str>, body: Option<&'static str>, draft: Option<bool>, prerelease: Option<bool>) -> ReleaseReq {
    ReleaseReq {
      tag_name: tag,
      target_commitish: commit,
      name: name,
      body: body,
      draft: draft,
      prerelease: prerelease
    }
  }

  pub fn builder(tag: &'static str) -> ReleaseBuilder {
    ReleaseBuilder::new(tag)
  }
}

#[derive(Debug, RustcDecodable)]
pub struct DeploymentStatus {
  pub url: String,
  pub created_at: String,
  pub updated_at: String,
  pub state: State,
  pub target_url: String,
  pub description: String,
  pub id: u64,
  pub deployment_url: String,
  pub repository_url: String,
  pub creator: User
}

impl Encodable for DeploymentStatusReq {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      DeploymentStatusReq {
        state: ref this_state,
        target_url: ref this_target_url,
        description: ref this_description
      } => {
          encoder.emit_struct("DeploymentStatusReq", 1_usize, |encoder| {
              let mut index = 0;
              try!(encoder.emit_struct_field("state", index, |encoder| this_state.encode(encoder)));
              if this_target_url.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("target_url", index, |encoder| this_target_url.encode(encoder)));
              }
              if this_description.is_some() {
                  index += 1;
                  try!(encoder.emit_struct_field("description", index, |encoder| this_description.encode(encoder)));
              }
              Ok(())
          })
      }
    }
  }
}

#[derive(Default)]
pub struct DeploymentStatusReqBuilder {
  state: State,
  target_url: Option<&'static str>,
  description: Option<&'static str>
}

impl DeploymentStatusReqBuilder {

  pub fn new(state: State) -> DeploymentStatusReqBuilder {
    DeploymentStatusReqBuilder {
      state: state,
      ..Default::default()
    }
  }

  pub fn target_url(&mut self, url: &'static str) -> &mut DeploymentStatusReqBuilder {
    self.target_url = Some(url);
    self
  }

  pub fn description(&mut self, desc: &'static str) -> &mut DeploymentStatusReqBuilder {
    self.description = Some(desc);
    self
  }

  pub fn build(&self) -> DeploymentStatusReq {
    DeploymentStatusReq {
      state: self.state.clone(),
      target_url: self.target_url,
      description: self.description
    }
  }
}

#[derive(Debug)]
pub struct DeploymentStatusReq {
  state: State,
  target_url: Option<&'static str>,
  description: Option<&'static str>
}

impl DeploymentStatusReq {
  pub fn builder(state: State) -> DeploymentStatusReqBuilder {
    DeploymentStatusReqBuilder::new(state)
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Status {
  pub created_at: String,
  pub updated_at: String,
  pub state: State,
  pub target_url: String,
  pub description: String,
  pub id: u64,
  pub url: String,
  pub context: String,
  pub creator: User
}

impl Encodable for StatusReq {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      StatusReq {
        state: ref this_state,
        target_url: ref this_target_url,
        description: ref this_description,
        context: ref this_context
      } => {
        encoder.emit_struct("StatusReq", 1usize, |encoder| {
          try!(encoder.emit_struct_field("state", 0usize, |encoder| this_state.encode(encoder)));
          if this_target_url.is_some() {
            try!(encoder.emit_struct_field("target_url", 0usize, |encoder| this_target_url.encode(encoder)));
          }
          if this_description.is_some() {
            try!(encoder.emit_struct_field("description", 0usize, |encoder| this_description.encode(encoder)));
          }
          if this_context.is_some() {
            try!(encoder.emit_struct_field("context", 0usize, |encoder| this_context.encode(encoder)));
          }
          Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct StatusReq {
  state: State,
  target_url: Option<&'static str>,
  description: Option<&'static str>,
  context: Option<&'static str>
}

#[derive(Default)]
pub struct StatusBuilder {
  state: State,
  target_url: Option<&'static str>,
  description: Option<&'static str>,
  context: Option<&'static str>,
}

impl StatusBuilder {
  pub fn new(state: State) -> StatusBuilder {
    StatusBuilder {
      state: state,
      ..Default::default()
    }
  }

  pub fn target_url(&mut self, url: &'static str) -> &mut StatusBuilder {
    self.target_url = Some(url);
    self
  }

  pub fn description(&mut self, desc: &'static str) -> &mut StatusBuilder {
    self.description = Some(desc);
    self
  }

  pub fn context(&mut self, ctx: &'static str) -> &mut StatusBuilder {
    self.context = Some(ctx);
    self
  }

  pub fn build(&self) -> StatusReq {
    StatusReq::new(self.state.clone(), self.target_url, self.description, self.context)
  }
}

impl StatusReq {
  pub fn new(state: State, target_url: Option<&'static str>, descr: Option<&'static str>, context: Option<&'static str>) -> StatusReq {
    StatusReq {
      state: state,
      target_url: target_url,
      description: descr,
      context: context
    }
  }

  pub fn builder(state: State) -> StatusBuilder {
    StatusBuilder::new(state)
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Key {
  pub id: u64,
  pub key: String,
  pub title: String,
  pub verified: bool,
  pub created_at: String,
  pub read_only: bool
}

#[derive(Debug, RustcEncodable)]
pub struct KeyReq {
  pub title: &'static str,
  pub key: &'static str,
  pub read_only: bool
}

#[cfg(test)]
mod tests {
    use rustc_serialize::{json, Encodable};
    use std::collections::HashMap;
    use super::*;
    use super::super::statuses::State;

    fn test_encoding<E: Encodable>(tests: Vec<(E, &str)>) {
        for test in tests {
            match test {
                (k, v) => assert_eq!(json::encode::<E>(&k).unwrap(), v)
            }
        }
    }

    #[test]
    fn gist_reqs() {
        let mut files = HashMap::new();
        files.insert("foo", "bar");
        let tests = vec![
            (
                GistReq::new(None, true, files.clone()),
                r#"{"public":true,"files":{"foo":{"content":"bar"}}}"#
            ),
            (
                GistReq::new(Some("desc"), true, files.clone()),
                r#"{"description":"desc","public":true,"files":{"foo":{"content":"bar"}}}"#
            )
        ];
        test_encoding(tests);
    }

    #[test]
    fn deployment_reqs() {
        let tests = vec![
            (
                DeploymentReq::builder("test").build(),
                r#"{"ref":"test"}"#
            ),
            (
                DeploymentReq::builder("test").task("launchit").build(),
                r#"{"ref":"test","task":"launchit"}"#
            )
        ];
        test_encoding(tests)
    }

    #[test]
    fn deployment_status_reqs() {
        let tests = vec![
            (
                DeploymentStatusReq::builder(State::pending).build(),
                r#"{"state":"pending"}"#
            ),
            (
                DeploymentStatusReq::builder(State::pending).target_url("http://host.com").build(),
                r#"{"state":"pending","target_url":"http://host.com"}"#
            ),
            (
                DeploymentStatusReq::builder(State::pending).target_url("http://host.com").description("desc").build(),
                r#"{"state":"pending","target_url":"http://host.com","description":"desc"}"#
            ),
        ];
        test_encoding(tests)
    }

    #[test]
    fn pullreq_edits() {
        let tests = vec![
            (
                PullEdit::new(Some("test"), None, None),
                r#"{"title":"test"}"#
            ),
            (
                PullEdit::new(Some("test"), Some("desc"), None),
                r#"{"title":"test","body":"desc"}"#
            )
        ];
        test_encoding(tests)
    }
}

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OuterEvent {
    UrlVerification {
        token: String,
        challenge: String,
    },
    EventCallback {
        token: String,
        team_id: String,
        api_app_id: String,
        event: InnerEvent,
        authed_users: Option<Vec<String>>,
        event_id: String,
        event_time: u64,
    },
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InnerEvent {
    /// User clicked into your App Home
    /// Permissions: None
    AppHomeOpened {
        user: String,
        channel: String,
        event_ts: Option<f32>,
        tab: String,
        view: Option<Value>,
    },
    /// Subscribe to only the message events that mention your app or bot
    /// Permissions: app_mentions:read
    AppMention {
        user: String,
        text: String,
        ts: f32,
        channel: String,
        event_ts: u64,
    },
    /// Indicates your app's event subscriptions are being rate limited
    /// Permissions: None
    AppRateLimited {
        token: String,
        team_id: String,
        minute_rate_limited: u64,
        api_app_id: String,
    },
    /// User requested an app
    /// Permissions: admin.apps:read
    AppRequested {}, // I'll get back to this.
    /// Your Slack app was uninstalled.
    /// Permissions: None
    AppUninstalled {},
    /// A call was rejected
    /// No Documentation provided for this event
    /// Permissions: calls:read
    CallRejected { channel: Channel },
    /// A channel was archived
    /// Permissions: channels:read
    ChannelArchive { channel: String, user: String },
    /// A channel was created
    /// Permissions: channels:read
    ChannelCreated {},
    /// A channel was deleted
    /// Permissions: channels:read
    ChannelDeleted {},
    /// Bulk updates were made to a channel's history
    /// Permissions: channels:history
    ChannelHistoryChanged {},
    /// You left a channel
    /// Permissions: channels:read
    ChannelLeft {},
    /// A channel was renamed
    /// Permissions: channels:read
    ChannelRename {},
    /// A channel has been shared with an external workspace
    /// Permissions: channels:read
    ChannelShared {},
    /// A channel was unarchived
    /// Permissions: channels:read
    ChannelUnarchive {},
    /// A channel has been unshared with an external workspace
    /// Permissions: channels:read
    ChannelUnshared {},
    /// Do not Disturb settings changed for the current user
    /// Permissions: dnd:read
    DndUpdated {},
    /// Do not Disturb settings changed for a member
    /// Permissions: dnd:read
    DndUpdatedUser {},
    /// The workspace email domain has changed
    /// Permissions: team:read
    EmailDomainChanged {},
    /// A custom emoji has been added or changed
    /// Permissions: emoji:read
    EmojiChanged {},
    /// A file was changed
    /// Permissions: files:read
    FileChange {},
    /// A file comment was added
    /// Permissions: files:read
    FileCommentAdded {},
    /// A file comment was deleted
    /// Permissions: files:read
    FileCommentDeleted {},
    /// A file comment was edited
    /// Permissions: files:read
    FileCommentEdited {},
    /// A file was created
    /// Permissions: files:read
    FileCreated {},
    /// A file was deleted
    /// Permissions: files:read
    FileDeleted {},
    /// A file was made public
    /// Permissions: files:read
    FilePublic {},
    /// A file was shared
    /// Permissions: files:read
    FileShared {},
    /// A file was unshared
    /// Permissions: files:read
    FileUnshared {},
    /// An enterprise grid migration has finished on this workspace.
    /// Permissions: None
    GridMigrationFinished {},
    /// An enterprise grid migration has started on this workspace.
    /// Permissions: None
    GridMigrationStarted {},
    /// A private channel was archived
    /// Permissions: groups:read
    GroupArchive {},
    /// You closed a private channel
    /// Permissions: groups:read
    GroupClose {},
    /// A private channel was deleted
    /// Permissions: groups:read
    GroupDeleted {},
    /// Bulk updates were made to a private channel's history
    /// Permissions: groups:history
    GroupHistoryChanged {},
    /// You left a private channel
    /// Permissions: groups:read
    GroupLeft {},
    /// You created a group DM
    /// Permissions: groups:read
    GroupOpen {},
    /// A private channel was renamed
    /// Permissions: groups:read
    GroupRename {},
    /// A private channel was unarchived
    /// Permissions: groups:read
    GroupUnarchive {},
    /// You closed a DM
    /// Permissions: im:read
    ImClose {},
    /// A DM was created
    /// Permissions: im:read
    ImCreated {},
    /// Bulk updates were made to a DM's history
    /// Permissions: im:history
    ImHistoryChanged {},
    /// You opened a DM
    /// Permissions: im:read
    ImOpen {},
    /// User requested an invite
    /// Permissions: admin.invites:read
    InviteRequested {},
    /// A message was posted containing one or more links relevant to your application
    /// Permissions: links:read
    LinkShared {},
    /// A user joined a public or private channel
    /// Permissions: channels:read
    MemberJoinedChannel {},
    /// A user left a public or private channel
    /// Permissions: channels:read
    MemberLeftChannel {},
    /// A message was sent to a channel
    /// Permissions: channels:history
    Message {
        client_msg_id: String,
        text: String,
        user: String,
        ts: String, // Float?
        team: String,
        channel: String,
        event_ts: String,
        channel_type: String, //Enum (im?)
        blocks: Vec<MessageBlock>,
    },
    /// message.app_home
    /// A user sent a message to your Slack app
    /// None
    /// message.channels
    /// A message was posted to a channel
    /// channels:history
    /// message.groups
    /// A message was posted to a private channel
    /// groups:history
    /// message.im
    /// A message was posted in a direct message channel
    /// im:history
    /// message.mpim
    /// A message was posted in a multiparty direct message channel
    /// mpim:history
    /// A pin was added to a channel
    /// pins:read
    PinAdded {},
    /// A pin was removed from a channel
    /// Permissions: pins:read
    PinRemoved {},
    /// A member has added an emoji reaction to an item
    /// Permissions: reactions:read
    ReactionAdded {},
    /// A member removed an emoji reaction
    /// Permissions: reactions:read
    ReactionRemoved {},
    /// Access to a set of resources was granted for your app
    /// Permissions: None
    ResourcesAdded {},
    /// Access to a set of resources was removed for your app
    /// Permissions: None
    ResourcesRemoved {},
    /// OAuth scopes were denied to your app
    /// Permissions: None
    ScopeDenied {},
    /// OAuth scopes were granted to your app
    /// Permissions: None
    ScopeGranted {},
    /// A member has starred an item
    /// Permissions: stars:read
    StarAdded {},
    /// A member removed a star
    /// Permissions: stars:read
    StarRemoved {},
    /// A User Group has been added to the workspace
    /// Permissions: usergroups:read
    SubteamCreated {},
    /// The membership of an existing User Group has changed
    /// Permissions: usergroups:read
    SubteamMembersChanged {},
    /// You have been added to a User Group
    /// Permissions: usergroups:read
    SubteamSelfAdded {},
    /// You have been removed from a User Group
    /// Permissions: usergroups:read
    SubteamSelfRemoved {},
    /// An existing User Group has been updated or its members changed
    /// Permissions: usergroups:read
    SubteamUpdated {},
    /// The workspace domain has changed
    /// Permissions: team:read
    TeamDomainChange {},
    /// A new member has joined
    /// Permissions: users:read
    TeamJoin {},
    /// The workspace name has changed
    /// Permissions: team:read
    TeamRename {},
    /// API tokens for your app were revoked.
    /// Permissions: None
    TokensRevoked {},
    /// Verifies ownership of an Events API Request URL
    /// Permissions: None
    UrlVerification {},
    /// A member's data has changed
    /// Permissions: users:read
    UserChange {},
    /// User resource was denied to your app
    /// Permissions: None
    UserResourceDenied {},
    /// User resource was granted to your app
    /// Permissions: None
    UserResourceGranted {},
    /// User resource was removed from your app
    /// Permissions: None
    UserResourceRemoved {},
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Channel {
    id: String,
    name: String,
    created: u64,
    creator: User,
}

type User = String;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageBlock {
    RichText {
        block_id: String,
        elements: Vec<RichTextElement>,
    },
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextElement {
    RichTextSection {
        elements: Vec<RichTextSectionElement>,
    },
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextSectionElement {
    Text { text: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    // use serde_json::{map::Map, Value};

    #[test]
    fn url_verification_works() {
        let challenge = "{
            \"token\": \"Jhj5dZrVaK7ZwHHjRyZWjbDl\",
            \"challenge\": \"3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P\",
            \"type\": \"url_verification\"
        }";
        let challenge: OuterEvent = serde_json::from_str(challenge).unwrap();
        assert_eq!(
            OuterEvent::UrlVerification {
                token: "Jhj5dZrVaK7ZwHHjRyZWjbDl".into(),
                challenge: "3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P".into()
            },
            challenge
        );
    }

    #[test]
    fn event_callback_works() {
        let callback = "{
            \"token\": \"XXYYZZ\",
            \"team_id\": \"TXXXXXXXX\",
            \"api_app_id\": \"AXXXXXXXXX\",
            \"event\": {
                    \"type\": \"pin_added\",
                    \"event_ts\": \"1234567890.123456\",
                    \"user\": \"UXXXXXXX1\" 
            },
            \"type\": \"event_callback\",
            \"authed_users\": [
                    \"UXXXXXXX1\",
                    \"UXXXXXXX2\"
            ],
            \"event_id\": \"Ev08MFMKH6\",
            \"event_time\": 1234567890
    }";
        let callback: OuterEvent = serde_json::from_str(callback).unwrap();
        assert_eq!(
            OuterEvent::EventCallback {
                token: "XXYYZZ".into(),
                team_id: "TXXXXXXXX".into(),
                api_app_id: "AXXXXXXXXX".into(),
                event: InnerEvent::PinAdded {},
                authed_users: Some(vec!["UXXXXXXX1".into(), "UXXXXXXX2".into()]),
                event_id: "Ev08MFMKH6".into(),
                event_time: 1_234_567_890
            },
            callback
        );
    }

    #[test]
    fn app_home_opened_works() {
        let callback = "{\"token\":\"gDimgAnOYefZ58jniKrv8BNA\",\"team_id\":\"T010346TVPH\",\"api_app_id\":\"A0103EF7Y3G\",\"event\":{\"type\":\"app_home_opened\",\"user\":\"U0103ED6A22\",\"channel\":\"D0103EVPKTQ\",\"tab\":\"messages\"},\"type\":\"event_callback\",\"event_id\":\"Ev0101E9ELH2\",\"event_time\":1584339448}";
        let callback: OuterEvent = serde_json::from_str(callback).unwrap();
        assert_eq!(
            OuterEvent::EventCallback {
                token: "gDimgAnOYefZ58jniKrv8BNA".into(),
                team_id: "T010346TVPH".into(),
                api_app_id: "A0103EF7Y3G".into(),
                event: InnerEvent::AppHomeOpened {
                    user: "U0103ED6A22".into(),
                    channel: "D0103EVPKTQ".into(),
                    event_ts: None,
                    tab: "messages".into(),
                    view: None
                },
                authed_users: None,
                event_id: "Ev0101E9ELH2".into(),
                event_time: 1_584_339_448
            },
            callback
        );
    }

    #[test]
    fn message_works() {
        let callback = "{
            \"token\":\"gDimgAnOYefZ58jniKrv8BNA\",
            \"team_id\":\"T010346TVPH\",
            \"api_app_id\":\"A0103EF7Y3G\",
            \"event\":{
                \"type\":\"message\",
                \"client_msg_id\":\"a5899740-233f-4656-8469-5f88c5b8db27\",
                \"text\":\"hello?\",
                \"user\":\"U0103ED6A22\",
                \"ts\":\"1584339455.000200\",
                \"team\":\"T010346TVPH\",
                \"channel\":\"D0103EVPKTQ\",
                \"event_ts\":\"1584339455.000200\",
                \"channel_type\":\"im\",
                \"blocks\":[{
                    \"type\":\"rich_text\",
                    \"block_id\":\"XCSy\",
                    \"elements\":[{
                        \"type\":\"rich_text_section\",
                        \"elements\":[{
                                \"type\":\"text\",
                                \"text\":\"hello?\"
                            }]
                        }]
                }]
            },
            \"type\":\"event_callback\",
            \"event_id\":\"Ev0103PNN1L7\",
            \"event_time\":1584339455,
            \"authed_users\":[\"U01018PDSNL\"]
        }";
        let callback: OuterEvent = serde_json::from_str(callback).unwrap();
        assert_eq!(
            OuterEvent::EventCallback {
                token: "gDimgAnOYefZ58jniKrv8BNA".into(),
                team_id: "T010346TVPH".into(),
                api_app_id: "A0103EF7Y3G".into(),
                event: InnerEvent::Message {
                    client_msg_id: "a5899740-233f-4656-8469-5f88c5b8db27".into(),
                    text: "hello?".into(),
                    user: "U0103ED6A22".into(),
                    ts: "1584339455.000200".into(),
                    team: "T010346TVPH".into(),
                    channel: "D0103EVPKTQ".into(),
                    event_ts: "1584339455.000200".into(),
                    channel_type: "im".into(),
                    blocks: vec![MessageBlock::RichText {
                        block_id: "XCSy".into(),
                        elements: vec![RichTextElement::RichTextSection {
                            elements: vec![RichTextSectionElement::Text {
                                text: "hello?".into()
                            }]
                        }]
                    }]
                },
                authed_users: Some(vec!["U01018PDSNL".into()]),
                event_id: "Ev0103PNN1L7".into(),
                event_time: 1_584_339_455
            },
            callback
        );
    }
}

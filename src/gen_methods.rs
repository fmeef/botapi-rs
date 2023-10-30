use crate::bot::Part;
use crate::{
    bot::{ApiError, Bot, BotResult, Response},
    gen_types::*,
};
use anyhow::Result;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Debug)]
struct ApproveChatJoinRequestOpts {
    chat_id: i64,
    user_id: i64,
}
#[derive(Serialize, Debug)]
struct DeleteForumTopicOpts {
    chat_id: i64,
    message_thread_id: i64,
}
#[derive(Serialize, Debug)]
struct SendVideoNoteOpts {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    video_note: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct PinChatMessageOpts {
    chat_id: i64,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}
#[derive(Serialize, Debug)]
struct SendAudioOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    audio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct SetMyCommandsOpts<'a> {
    commands: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct GetMeOpts();
#[derive(Serialize, Debug)]
struct GetCustomEmojiStickersOpts {
    custom_emoji_ids: String,
}
#[derive(Serialize, Debug)]
struct AnswerWebAppQueryOpts<'a> {
    web_app_query_id: &'a str,
    result: String,
}
#[derive(Serialize, Debug)]
struct RestrictChatMemberOpts {
    chat_id: i64,
    user_id: i64,
    permissions: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
}
#[derive(Serialize, Debug)]
struct UnpinChatMessageOpts {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
}
#[derive(Serialize, Debug)]
struct SetChatAdministratorCustomTitleOpts<'a> {
    chat_id: i64,
    user_id: i64,
    custom_title: &'a str,
}
#[derive(Serialize, Debug)]
struct SendDocumentOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    document: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_content_type_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct EditGeneralForumTopicOpts<'a> {
    chat_id: i64,
    name: &'a str,
}
#[derive(Serialize, Debug)]
struct SetCustomEmojiStickerSetThumbnailOpts<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_emoji_id: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct SendMediaGroupOpts {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
}
#[derive(Serialize, Debug)]
struct SetMyDescriptionOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct SetStickerSetThumbnailOpts<'a> {
    name: &'a str,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
}
#[derive(Serialize, Debug)]
struct UploadStickerFileOpts<'a> {
    user_id: i64,
    sticker: String,
    sticker_format: &'a str,
}
#[derive(Serialize, Debug)]
struct EditMessageReplyMarkupOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct GetWebhookInfoOpts();
#[derive(Serialize, Debug)]
struct SetChatPermissionsOpts {
    chat_id: i64,
    permissions: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
}
#[derive(Serialize, Debug)]
struct UnpinAllForumTopicMessagesOpts {
    chat_id: i64,
    message_thread_id: i64,
}
#[derive(Serialize, Debug)]
struct GetChatOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct SetStickerEmojiListOpts<'a> {
    sticker: &'a str,
    emoji_list: String,
}
#[derive(Serialize, Debug)]
struct ForwardMessageOpts {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    from_chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    message_id: i64,
}
#[derive(Serialize, Debug)]
struct SendContactOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    phone_number: &'a str,
    first_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct DeleteChatPhotoOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct GetChatMemberOpts {
    chat_id: i64,
    user_id: i64,
}
#[derive(Serialize, Debug)]
struct HideGeneralForumTopicOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct EditMessageTextOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct SetStickerPositionInSetOpts<'a> {
    sticker: &'a str,
    position: i64,
}
#[derive(Serialize, Debug)]
struct SendVideoOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    video: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct AnswerInlineQueryOpts<'a> {
    inline_query_id: &'a str,
    results: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<String>,
}
#[derive(Serialize, Debug)]
struct BanChatSenderChatOpts {
    chat_id: i64,
    sender_chat_id: i64,
}
#[derive(Serialize, Debug)]
struct PromoteChatMemberOpts {
    chat_id: i64,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_topics: Option<bool>,
}
#[derive(Serialize, Debug)]
struct DeleteMyCommandsOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct ExportChatInviteLinkOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct GetChatMenuButtonOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
}
#[derive(Serialize, Debug)]
struct EditMessageLiveLocationOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
    latitude: ::ordered_float::OrderedFloat<f64>,
    longitude: ::ordered_float::OrderedFloat<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<::ordered_float::OrderedFloat<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct DeleteMessageOpts {
    chat_id: i64,
    message_id: i64,
}
#[derive(Serialize, Debug)]
struct SetStickerMaskPositionOpts<'a> {
    sticker: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<String>,
}
#[derive(Serialize, Debug)]
struct SetChatTitleOpts<'a> {
    chat_id: i64,
    title: &'a str,
}
#[derive(Serialize, Debug)]
struct SetPassportDataErrorsOpts {
    user_id: i64,
    errors: String,
}
#[derive(Serialize, Debug)]
struct CloseOpts();
#[derive(Serialize, Debug)]
struct SendStickerOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct DeleteWebhookOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
}
#[derive(Serialize, Debug)]
struct SetMyDefaultAdministratorRightsOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    rights: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}
#[derive(Serialize, Debug)]
struct UnpinAllGeneralForumTopicMessagesOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct EditChatInviteLinkOpts<'a> {
    chat_id: i64,
    invite_link: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}
#[derive(Serialize, Debug)]
struct EditMessageCaptionOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct DeleteStickerFromSetOpts<'a> {
    sticker: &'a str,
}
#[derive(Serialize, Debug)]
struct SendAnimationOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    animation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct SetChatStickerSetOpts<'a> {
    chat_id: i64,
    sticker_set_name: &'a str,
}
#[derive(Serialize, Debug)]
struct SendChatActionOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    action: &'a str,
}
#[derive(Serialize, Debug)]
struct GetFileOpts<'a> {
    file_id: &'a str,
}
#[derive(Serialize, Debug)]
struct AddStickerToSetOpts<'a> {
    user_id: i64,
    name: &'a str,
    sticker: String,
}
#[derive(Serialize, Debug)]
struct GetMyNameOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct SetStickerSetTitleOpts<'a> {
    name: &'a str,
    title: &'a str,
}
#[derive(Serialize, Debug)]
struct CloseForumTopicOpts {
    chat_id: i64,
    message_thread_id: i64,
}
#[derive(Serialize, Debug)]
struct GetMyCommandsOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct CopyMessageOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    from_chat_id: i64,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct UnpinAllChatMessagesOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct GetChatMemberCountOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct AnswerPreCheckoutQueryOpts<'a> {
    pre_checkout_query_id: &'a str,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct CreateChatInviteLinkOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}
#[derive(Serialize, Debug)]
struct EditForumTopicOpts<'a> {
    chat_id: i64,
    message_thread_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct SetWebhookOpts<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_token: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct LogOutOpts();
#[derive(Serialize, Debug)]
struct RevokeChatInviteLinkOpts<'a> {
    chat_id: i64,
    invite_link: &'a str,
}
#[derive(Serialize, Debug)]
struct ReopenForumTopicOpts {
    chat_id: i64,
    message_thread_id: i64,
}
#[derive(Serialize, Debug)]
struct GetMyShortDescriptionOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct GetMyDefaultAdministratorRightsOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}
#[derive(Serialize, Debug)]
struct SetStickerKeywordsOpts<'a> {
    sticker: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<String>,
}
#[derive(Serialize, Debug)]
struct SetChatMenuButtonOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_button: Option<String>,
}
#[derive(Serialize, Debug)]
struct EditMessageMediaOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
    media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct ReopenGeneralForumTopicOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct GetGameHighScoresOpts<'a> {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct CreateForumTopicOpts<'a> {
    chat_id: i64,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct SendInvoiceOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    title: &'a str,
    description: &'a str,
    payload: &'a str,
    provider_token: &'a str,
    currency: &'a str,
    prices: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_parameter: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct LeaveChatOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct DeleteChatStickerSetOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct GetStickerSetOpts<'a> {
    name: &'a str,
}
#[derive(Serialize, Debug)]
struct GetMyDescriptionOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct SendPhotoOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    photo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct CloseGeneralForumTopicOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct SetChatDescriptionOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct GetChatAdministratorsOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct GetForumTopicIconStickersOpts();
#[derive(Serialize, Debug)]
struct UnhideGeneralForumTopicOpts {
    chat_id: i64,
}
#[derive(Serialize, Debug)]
struct StopPollOpts {
    chat_id: i64,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct BanChatMemberOpts {
    chat_id: i64,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_messages: Option<bool>,
}
#[derive(Serialize, Debug)]
struct SendVenueOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    latitude: ::ordered_float::OrderedFloat<f64>,
    longitude: ::ordered_float::OrderedFloat<f64>,
    title: &'a str,
    address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct DeclineChatJoinRequestOpts {
    chat_id: i64,
    user_id: i64,
}
#[derive(Serialize, Debug)]
struct SetChatPhotoOpts {
    chat_id: i64,
    photo: String,
}
#[derive(Serialize, Debug)]
struct CreateInvoiceLinkOpts<'a> {
    title: &'a str,
    description: &'a str,
    payload: &'a str,
    provider_token: &'a str,
    currency: &'a str,
    prices: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
}
#[derive(Serialize, Debug)]
struct StopMessageLiveLocationOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct SendGameOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    game_short_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct UnbanChatMemberOpts {
    chat_id: i64,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_if_banned: Option<bool>,
}
#[derive(Serialize, Debug)]
struct SendVoiceOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct SendDiceOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct AnswerCallbackQueryOpts<'a> {
    callback_query_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<i64>,
}
#[derive(Serialize, Debug)]
struct UnbanChatSenderChatOpts {
    chat_id: i64,
    sender_chat_id: i64,
}
#[derive(Serialize, Debug)]
struct SetMyNameOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct DeleteStickerSetOpts<'a> {
    name: &'a str,
}
#[derive(Serialize, Debug)]
struct SendPollOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    question: &'a str,
    options: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tg_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct GetUserProfilePhotosOpts {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
#[derive(Serialize, Debug)]
struct SetMyShortDescriptionOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct CreateNewStickerSetOpts<'a> {
    user_id: i64,
    name: &'a str,
    title: &'a str,
    stickers: String,
    sticker_format: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    needs_repainting: Option<bool>,
}
#[derive(Serialize, Debug)]
struct AnswerShippingQueryOpts<'a> {
    shipping_query_id: &'a str,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<&'a str>,
}
#[derive(Serialize, Debug)]
struct SendMessageOpts<'a> {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct GetUpdatesOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<String>,
}
#[derive(Serialize, Debug)]
struct SendLocationOpts {
    chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    latitude: ::ordered_float::OrderedFloat<f64>,
    longitude: ::ordered_float::OrderedFloat<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<::ordered_float::OrderedFloat<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<String>,
}
#[derive(Serialize, Debug)]
struct SetGameScoreOpts<'a> {
    user_id: i64,
    score: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<&'a str>,
}
pub struct CallApproveChatJoinRequest<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
}
impl<'a> CallApproveChatJoinRequest<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .approve_chat_join_request(self.chat_id, self.user_id)
            .await
    }
}
pub struct CallDeleteForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: i64,
}
impl<'a> CallDeleteForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread of the forum topic"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = message_thread_id;
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a i64 {
        &self.message_thread_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .delete_forum_topic(self.chat_id, self.message_thread_id)
            .await
    }
}
pub struct CallSendVideoNote<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    video_note: FileData,
    duration: Option<i64>,
    length: Option<i64>,
    thumbnail: Option<FileData>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendVideoNote<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files. Sending video notes by a URL is currently unsupported"]
    pub fn video_note(mut self, video_note: FileData) -> Self {
        self.video_note = video_note;
        self
    }
    pub fn get_video_note(&'a self) -> &'a FileData {
        &self.video_note
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Duration of sent video in seconds"]
    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }
    pub fn get_duration(&'a self) -> &'a Option<i64> {
        &self.duration
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Video width and height, i.e. diameter of the video message"]
    pub fn length(mut self, length: i64) -> Self {
        self.length = Some(length);
        self
    }
    pub fn get_length(&'a self) -> &'a Option<i64> {
        &self.length
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass \"attach://<file_attach_name>\" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn thumbnail(mut self, thumbnail: FileData) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }
    pub fn get_thumbnail(&'a self) -> &'a Option<FileData> {
        &self.thumbnail
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_video_note(
                self.chat_id,
                self.message_thread_id,
                self.video_note,
                self.duration,
                self.length,
                self.thumbnail,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallPinChatMessage<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_id: i64,
    disable_notification: Option<bool>,
}
impl<'a> CallPinChatMessage<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Identifier of a message to pin"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }
    pub fn get_message_id(&'a self) -> &'a i64 {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .pin_chat_message(self.chat_id, self.message_id, self.disable_notification)
            .await
    }
}
pub struct CallSendAudio<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    audio: FileData,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    duration: Option<i64>,
    performer: Option<&'a str>,
    title: Option<&'a str>,
    thumbnail: Option<FileData>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendAudio<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn audio(mut self, audio: FileData) -> Self {
        self.audio = audio;
        self
    }
    pub fn get_audio(&'a self) -> &'a FileData {
        &self.audio
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Audio caption, 0-1024 characters after entities parsing"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the audio caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Duration of the audio in seconds"]
    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }
    pub fn get_duration(&'a self) -> &'a Option<i64> {
        &self.duration
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Performer"]
    pub fn performer(mut self, performer: &'a str) -> Self {
        self.performer = Some(performer);
        self
    }
    pub fn get_performer(&'a self) -> &'a Option<&'a str> {
        &self.performer
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Track name"]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
    pub fn get_title(&'a self) -> &'a Option<&'a str> {
        &self.title
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass \"attach://<file_attach_name>\" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn thumbnail(mut self, thumbnail: FileData) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }
    pub fn get_thumbnail(&'a self) -> &'a Option<FileData> {
        &self.thumbnail
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_audio(
                self.chat_id,
                self.message_thread_id,
                self.audio,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.duration,
                self.performer,
                self.title,
                self.thumbnail,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallSetMyCommands<'a> {
    bot: &'a Bot,
    commands: &'a Vec<BotCommand>,
    scope: Option<&'a BotCommandScope>,
    language_code: Option<&'a str>,
}
impl<'a> CallSetMyCommands<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified."]
    pub fn commands(mut self, commands: &'a Vec<BotCommand>) -> Self {
        self.commands = commands;
        self
    }
    pub fn get_commands(&'a self) -> &'a &'a Vec<BotCommand> {
        &self.commands
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault."]
    pub fn scope(mut self, scope: &'a BotCommandScope) -> Self {
        self.scope = Some(scope);
        self
    }
    pub fn get_scope(&'a self) -> &'a Option<&'a BotCommandScope> {
        &self.scope
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands"]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_my_commands(&self.commands, self.scope, self.language_code)
            .await
    }
}
pub struct CallGetMe<'a> {
    bot: &'a Bot,
}
impl<'a> CallGetMe<'a> {
    pub async fn build(self) -> BotResult<User> {
        self.bot.get_me().await
    }
}
pub struct CallGetCustomEmojiStickers<'a> {
    bot: &'a Bot,
    custom_emoji_ids: &'a Vec<String>,
}
impl<'a> CallGetCustomEmojiStickers<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "List of custom emoji identifiers. At most 200 custom emoji identifiers can be specified."]
    pub fn custom_emoji_ids(mut self, custom_emoji_ids: &'a Vec<String>) -> Self {
        self.custom_emoji_ids = custom_emoji_ids;
        self
    }
    pub fn get_custom_emoji_ids(&'a self) -> &'a &'a Vec<String> {
        &self.custom_emoji_ids
    }
    pub async fn build(self) -> BotResult<Vec<Sticker>> {
        self.bot
            .get_custom_emoji_stickers(&self.custom_emoji_ids)
            .await
    }
}
pub struct CallAnswerWebAppQuery<'a> {
    bot: &'a Bot,
    web_app_query_id: &'a str,
    result: &'a InlineQueryResult,
}
impl<'a> CallAnswerWebAppQuery<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the query to be answered"]
    pub fn web_app_query_id(mut self, web_app_query_id: &'a str) -> Self {
        self.web_app_query_id = web_app_query_id;
        self
    }
    pub fn get_web_app_query_id(&'a self) -> &'a &'a str {
        &self.web_app_query_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object describing the message to be sent"]
    pub fn result(mut self, result: &'a InlineQueryResult) -> Self {
        self.result = result;
        self
    }
    pub fn get_result(&'a self) -> &'a &'a InlineQueryResult {
        &self.result
    }
    pub async fn build(self) -> BotResult<SentWebAppMessage> {
        self.bot
            .answer_web_app_query(&self.web_app_query_id, &self.result)
            .await
    }
}
pub struct CallRestrictChatMember<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
    permissions: &'a ChatPermissions,
    use_independent_chat_permissions: Option<bool>,
    until_date: Option<i64>,
}
impl<'a> CallRestrictChatMember<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for new user permissions"]
    pub fn permissions(mut self, permissions: &'a ChatPermissions) -> Self {
        self.permissions = permissions;
        self
    }
    pub fn get_permissions(&'a self) -> &'a &'a ChatPermissions {
        &self.permissions
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if chat permissions are set independently. Otherwise, the can_send_other_messages and can_add_web_page_previews permissions will imply the can_send_messages, can_send_audios, can_send_documents, can_send_photos, can_send_videos, can_send_video_notes, and can_send_voice_notes permissions; the can_send_polls permission will imply the can_send_messages permission."]
    pub fn use_independent_chat_permissions(
        mut self,
        use_independent_chat_permissions: bool,
    ) -> Self {
        self.use_independent_chat_permissions = Some(use_independent_chat_permissions);
        self
    }
    pub fn get_use_independent_chat_permissions(&'a self) -> &'a Option<bool> {
        &self.use_independent_chat_permissions
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever"]
    pub fn until_date(mut self, until_date: i64) -> Self {
        self.until_date = Some(until_date);
        self
    }
    pub fn get_until_date(&'a self) -> &'a Option<i64> {
        &self.until_date
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .restrict_chat_member(
                self.chat_id,
                self.user_id,
                &self.permissions,
                self.use_independent_chat_permissions,
                self.until_date,
            )
            .await
    }
}
pub struct CallUnpinChatMessage<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_id: Option<i64>,
}
impl<'a> CallUnpinChatMessage<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned."]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .unpin_chat_message(self.chat_id, self.message_id)
            .await
    }
}
pub struct CallSetChatAdministratorCustomTitle<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
    custom_title: &'a str,
}
impl<'a> CallSetChatAdministratorCustomTitle<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New custom title for the administrator; 0-16 characters, emoji are not allowed"]
    pub fn custom_title(mut self, custom_title: &'a str) -> Self {
        self.custom_title = custom_title;
        self
    }
    pub fn get_custom_title(&'a self) -> &'a &'a str {
        &self.custom_title
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_chat_administrator_custom_title(self.chat_id, self.user_id, &self.custom_title)
            .await
    }
}
pub struct CallSendDocument<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    document: FileData,
    thumbnail: Option<FileData>,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendDocument<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn document(mut self, document: FileData) -> Self {
        self.document = document;
        self
    }
    pub fn get_document(&'a self) -> &'a FileData {
        &self.document
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass \"attach://<file_attach_name>\" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn thumbnail(mut self, thumbnail: FileData) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }
    pub fn get_thumbnail(&'a self) -> &'a Option<FileData> {
        &self.thumbnail
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Document caption (may also be used when resending documents by file_id), 0-1024 characters after entities parsing"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the document caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Disables automatic server-side content type detection for files uploaded using multipart/form-data"]
    pub fn disable_content_type_detection(mut self, disable_content_type_detection: bool) -> Self {
        self.disable_content_type_detection = Some(disable_content_type_detection);
        self
    }
    pub fn get_disable_content_type_detection(&'a self) -> &'a Option<bool> {
        &self.disable_content_type_detection
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_document(
                self.chat_id,
                self.message_thread_id,
                self.document,
                self.thumbnail,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.disable_content_type_detection,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallEditGeneralForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
    name: &'a str,
}
impl<'a> CallEditGeneralForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New topic name, 1-128 characters"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .edit_general_forum_topic(self.chat_id, &self.name)
            .await
    }
}
pub struct CallSetCustomEmojiStickerSetThumbnail<'a> {
    bot: &'a Bot,
    name: &'a str,
    custom_emoji_id: Option<&'a str>,
}
impl<'a> CallSetCustomEmojiStickerSetThumbnail<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker set name"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail."]
    pub fn custom_emoji_id(mut self, custom_emoji_id: &'a str) -> Self {
        self.custom_emoji_id = Some(custom_emoji_id);
        self
    }
    pub fn get_custom_emoji_id(&'a self) -> &'a Option<&'a str> {
        &self.custom_emoji_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_custom_emoji_sticker_set_thumbnail(&self.name, self.custom_emoji_id)
            .await
    }
}
pub struct CallSendMediaGroup<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    media: &'a Vec<EMedia>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
}
impl<'a> CallSendMediaGroup<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized array describing messages to be sent, must include 2-10 items"]
    pub fn media(mut self, media: &'a Vec<EMedia>) -> Self {
        self.media = media;
        self
    }
    pub fn get_media(&'a self) -> &'a &'a Vec<EMedia> {
        &self.media
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends messages silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent messages from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the messages are a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    pub async fn build(self) -> BotResult<Vec<Message>> {
        self.bot
            .send_media_group(
                self.chat_id,
                self.message_thread_id,
                &self.media,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
            )
            .await
    }
}
pub struct CallSetMyDescription<'a> {
    bot: &'a Bot,
    description: Option<&'a str>,
    language_code: Option<&'a str>,
}
impl<'a> CallSetMyDescription<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language."]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
    pub fn get_description(&'a self) -> &'a Option<&'a str> {
        &self.description
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description."]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_my_description(self.description, self.language_code)
            .await
    }
}
pub struct CallSetStickerSetThumbnail<'a> {
    bot: &'a Bot,
    name: &'a str,
    user_id: i64,
    thumbnail: Option<FileData>,
}
impl<'a> CallSetStickerSetThumbnail<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker set name"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "User identifier of the sticker set owner"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A .WEBP or .PNG image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a .TGS animation with a thumbnail up to 32 kilobytes in size (see https://core.telegram.org/stickers#animated-sticker-requirements for animated sticker technical requirements), or a WEBM video with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#video-sticker-requirements for video sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files. Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail."]
    pub fn thumbnail(mut self, thumbnail: FileData) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }
    pub fn get_thumbnail(&'a self) -> &'a Option<FileData> {
        &self.thumbnail
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_sticker_set_thumbnail(&self.name, self.user_id, self.thumbnail)
            .await
    }
}
pub struct CallUploadStickerFile<'a> {
    bot: &'a Bot,
    user_id: i64,
    sticker: FileData,
    sticker_format: &'a str,
}
impl<'a> CallUploadStickerFile<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "User identifier of sticker file owner"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See https://core.telegram.org/stickers for technical requirements. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn sticker(mut self, sticker: FileData) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a FileData {
        &self.sticker
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Format of the sticker, must be one of \"static\", \"animated\", \"video\""]
    pub fn sticker_format(mut self, sticker_format: &'a str) -> Self {
        self.sticker_format = sticker_format;
        self
    }
    pub fn get_sticker_format(&'a self) -> &'a &'a str {
        &self.sticker_format
    }
    pub async fn build(self) -> BotResult<File> {
        self.bot
            .upload_sticker_file(self.user_id, self.sticker, &self.sticker_format)
            .await
    }
}
pub struct CallEditMessageReplyMarkup<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallEditMessageReplyMarkup<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the message to edit"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for an inline keyboard."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<MessageBool> {
        self.bot
            .edit_message_reply_markup(
                self.chat_id,
                self.message_id,
                self.inline_message_id,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallGetWebhookInfo<'a> {
    bot: &'a Bot,
}
impl<'a> CallGetWebhookInfo<'a> {
    pub async fn build(self) -> BotResult<WebhookInfo> {
        self.bot.get_webhook_info().await
    }
}
pub struct CallSetChatPermissions<'a> {
    bot: &'a Bot,
    chat_id: i64,
    permissions: &'a ChatPermissions,
    use_independent_chat_permissions: Option<bool>,
}
impl<'a> CallSetChatPermissions<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for new default chat permissions"]
    pub fn permissions(mut self, permissions: &'a ChatPermissions) -> Self {
        self.permissions = permissions;
        self
    }
    pub fn get_permissions(&'a self) -> &'a &'a ChatPermissions {
        &self.permissions
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if chat permissions are set independently. Otherwise, the can_send_other_messages and can_add_web_page_previews permissions will imply the can_send_messages, can_send_audios, can_send_documents, can_send_photos, can_send_videos, can_send_video_notes, and can_send_voice_notes permissions; the can_send_polls permission will imply the can_send_messages permission."]
    pub fn use_independent_chat_permissions(
        mut self,
        use_independent_chat_permissions: bool,
    ) -> Self {
        self.use_independent_chat_permissions = Some(use_independent_chat_permissions);
        self
    }
    pub fn get_use_independent_chat_permissions(&'a self) -> &'a Option<bool> {
        &self.use_independent_chat_permissions
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_chat_permissions(
                self.chat_id,
                &self.permissions,
                self.use_independent_chat_permissions,
            )
            .await
    }
}
pub struct CallUnpinAllForumTopicMessages<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: i64,
}
impl<'a> CallUnpinAllForumTopicMessages<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread of the forum topic"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = message_thread_id;
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a i64 {
        &self.message_thread_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .unpin_all_forum_topic_messages(self.chat_id, self.message_thread_id)
            .await
    }
}
pub struct CallGetChat<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallGetChat<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<Chat> {
        self.bot.get_chat(self.chat_id).await
    }
}
pub struct CallSetStickerEmojiList<'a> {
    bot: &'a Bot,
    sticker: &'a str,
    emoji_list: &'a Vec<String>,
}
impl<'a> CallSetStickerEmojiList<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "File identifier of the sticker"]
    pub fn sticker(mut self, sticker: &'a str) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a &'a str {
        &self.sticker
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of 1-20 emoji associated with the sticker"]
    pub fn emoji_list(mut self, emoji_list: &'a Vec<String>) -> Self {
        self.emoji_list = emoji_list;
        self
    }
    pub fn get_emoji_list(&'a self) -> &'a &'a Vec<String> {
        &self.emoji_list
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_sticker_emoji_list(&self.sticker, &self.emoji_list)
            .await
    }
}
pub struct CallForwardMessage<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    from_chat_id: i64,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    message_id: i64,
}
impl<'a> CallForwardMessage<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)"]
    pub fn from_chat_id(mut self, from_chat_id: i64) -> Self {
        self.from_chat_id = from_chat_id;
        self
    }
    pub fn get_from_chat_id(&'a self) -> &'a i64 {
        &self.from_chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the forwarded message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Message identifier in the chat specified in from_chat_id"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }
    pub fn get_message_id(&'a self) -> &'a i64 {
        &self.message_id
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .forward_message(
                self.chat_id,
                self.message_thread_id,
                self.from_chat_id,
                self.disable_notification,
                self.protect_content,
                self.message_id,
            )
            .await
    }
}
pub struct CallSendContact<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    phone_number: &'a str,
    first_name: &'a str,
    last_name: Option<&'a str>,
    vcard: Option<&'a str>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendContact<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Contact's phone number"]
    pub fn phone_number(mut self, phone_number: &'a str) -> Self {
        self.phone_number = phone_number;
        self
    }
    pub fn get_phone_number(&'a self) -> &'a &'a str {
        &self.phone_number
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Contact's first name"]
    pub fn first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = first_name;
        self
    }
    pub fn get_first_name(&'a self) -> &'a &'a str {
        &self.first_name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Contact's last name"]
    pub fn last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }
    pub fn get_last_name(&'a self) -> &'a Option<&'a str> {
        &self.last_name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional data about the contact in the form of a vCard, 0-2048 bytes"]
    pub fn vcard(mut self, vcard: &'a str) -> Self {
        self.vcard = Some(vcard);
        self
    }
    pub fn get_vcard(&'a self) -> &'a Option<&'a str> {
        &self.vcard
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_contact(
                self.chat_id,
                self.message_thread_id,
                &self.phone_number,
                &self.first_name,
                self.last_name,
                self.vcard,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallDeleteChatPhoto<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallDeleteChatPhoto<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.delete_chat_photo(self.chat_id).await
    }
}
pub struct CallGetChatMember<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
}
impl<'a> CallGetChatMember<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    pub async fn build(self) -> BotResult<ChatMember> {
        self.bot.get_chat_member(self.chat_id, self.user_id).await
    }
}
pub struct CallHideGeneralForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallHideGeneralForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.hide_general_forum_topic(self.chat_id).await
    }
}
pub struct CallEditMessageText<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    text: &'a str,
    parse_mode: Option<&'a str>,
    entities: Option<&'a Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallEditMessageText<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the message to edit"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New text of the message, 1-4096 characters after entities parsing"]
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }
    pub fn get_text(&'a self) -> &'a &'a str {
        &self.text
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the message text. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in message text, which can be specified instead of parse_mode"]
    pub fn entities(mut self, entities: &'a Vec<MessageEntity>) -> Self {
        self.entities = Some(entities);
        self
    }
    pub fn get_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Disables link previews for links in this message"]
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }
    pub fn get_disable_web_page_preview(&'a self) -> &'a Option<bool> {
        &self.disable_web_page_preview
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for an inline keyboard."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<MessageBool> {
        self.bot
            .edit_message_text(
                self.chat_id,
                self.message_id,
                self.inline_message_id,
                &self.text,
                self.parse_mode,
                self.entities,
                self.disable_web_page_preview,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallSetStickerPositionInSet<'a> {
    bot: &'a Bot,
    sticker: &'a str,
    position: i64,
}
impl<'a> CallSetStickerPositionInSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "File identifier of the sticker"]
    pub fn sticker(mut self, sticker: &'a str) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a &'a str {
        &self.sticker
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New sticker position in the set, zero-based"]
    pub fn position(mut self, position: i64) -> Self {
        self.position = position;
        self
    }
    pub fn get_position(&'a self) -> &'a i64 {
        &self.position
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_sticker_position_in_set(&self.sticker, self.position)
            .await
    }
}
pub struct CallSendVideo<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    video: FileData,
    duration: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    thumbnail: Option<FileData>,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
    supports_streaming: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendVideo<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn video(mut self, video: FileData) -> Self {
        self.video = video;
        self
    }
    pub fn get_video(&'a self) -> &'a FileData {
        &self.video
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Duration of sent video in seconds"]
    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }
    pub fn get_duration(&'a self) -> &'a Option<i64> {
        &self.duration
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Video width"]
    pub fn width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }
    pub fn get_width(&'a self) -> &'a Option<i64> {
        &self.width
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Video height"]
    pub fn height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }
    pub fn get_height(&'a self) -> &'a Option<i64> {
        &self.height
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass \"attach://<file_attach_name>\" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn thumbnail(mut self, thumbnail: FileData) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }
    pub fn get_thumbnail(&'a self) -> &'a Option<FileData> {
        &self.thumbnail
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Video caption (may also be used when resending videos by file_id), 0-1024 characters after entities parsing"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the video caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the video needs to be covered with a spoiler animation"]
    pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }
    pub fn get_has_spoiler(&'a self) -> &'a Option<bool> {
        &self.has_spoiler
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the uploaded video is suitable for streaming"]
    pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }
    pub fn get_supports_streaming(&'a self) -> &'a Option<bool> {
        &self.supports_streaming
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_video(
                self.chat_id,
                self.message_thread_id,
                self.video,
                self.duration,
                self.width,
                self.height,
                self.thumbnail,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.has_spoiler,
                self.supports_streaming,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallAnswerInlineQuery<'a> {
    bot: &'a Bot,
    inline_query_id: &'a str,
    results: &'a Vec<InlineQueryResult>,
    cache_time: Option<i64>,
    is_personal: Option<bool>,
    next_offset: Option<&'a str>,
    button: Option<&'a InlineQueryResultsButton>,
}
impl<'a> CallAnswerInlineQuery<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the answered query"]
    pub fn inline_query_id(mut self, inline_query_id: &'a str) -> Self {
        self.inline_query_id = inline_query_id;
        self
    }
    pub fn get_inline_query_id(&'a self) -> &'a &'a str {
        &self.inline_query_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized array of results for the inline query"]
    pub fn results(mut self, results: &'a Vec<InlineQueryResult>) -> Self {
        self.results = results;
        self
    }
    pub fn get_results(&'a self) -> &'a &'a Vec<InlineQueryResult> {
        &self.results
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300."]
    pub fn cache_time(mut self, cache_time: i64) -> Self {
        self.cache_time = Some(cache_time);
        self
    }
    pub fn get_cache_time(&'a self) -> &'a Option<i64> {
        &self.cache_time
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query."]
    pub fn is_personal(mut self, is_personal: bool) -> Self {
        self.is_personal = Some(is_personal);
        self
    }
    pub fn get_is_personal(&'a self) -> &'a Option<bool> {
        &self.is_personal
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes."]
    pub fn next_offset(mut self, next_offset: &'a str) -> Self {
        self.next_offset = Some(next_offset);
        self
    }
    pub fn get_next_offset(&'a self) -> &'a Option<&'a str> {
        &self.next_offset
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object describing a button to be shown above inline query results"]
    pub fn button(mut self, button: &'a InlineQueryResultsButton) -> Self {
        self.button = Some(button);
        self
    }
    pub fn get_button(&'a self) -> &'a Option<&'a InlineQueryResultsButton> {
        &self.button
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .answer_inline_query(
                &self.inline_query_id,
                &self.results,
                self.cache_time,
                self.is_personal,
                self.next_offset,
                self.button,
            )
            .await
    }
}
pub struct CallBanChatSenderChat<'a> {
    bot: &'a Bot,
    chat_id: i64,
    sender_chat_id: i64,
}
impl<'a> CallBanChatSenderChat<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target sender chat"]
    pub fn sender_chat_id(mut self, sender_chat_id: i64) -> Self {
        self.sender_chat_id = sender_chat_id;
        self
    }
    pub fn get_sender_chat_id(&'a self) -> &'a i64 {
        &self.sender_chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .ban_chat_sender_chat(self.chat_id, self.sender_chat_id)
            .await
    }
}
pub struct CallPromoteChatMember<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
    is_anonymous: Option<bool>,
    can_manage_chat: Option<bool>,
    can_delete_messages: Option<bool>,
    can_manage_video_chats: Option<bool>,
    can_restrict_members: Option<bool>,
    can_promote_members: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_pin_messages: Option<bool>,
    can_post_stories: Option<bool>,
    can_edit_stories: Option<bool>,
    can_delete_stories: Option<bool>,
    can_manage_topics: Option<bool>,
}
impl<'a> CallPromoteChatMember<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator's presence in the chat is hidden"]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }
    pub fn get_is_anonymous(&'a self) -> &'a Option<bool> {
        &self.is_anonymous
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can access the chat event log, boost list in channels, see channel members, report spam messages, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege"]
    pub fn can_manage_chat(mut self, can_manage_chat: bool) -> Self {
        self.can_manage_chat = Some(can_manage_chat);
        self
    }
    pub fn get_can_manage_chat(&'a self) -> &'a Option<bool> {
        &self.can_manage_chat
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can delete messages of other users"]
    pub fn can_delete_messages(mut self, can_delete_messages: bool) -> Self {
        self.can_delete_messages = Some(can_delete_messages);
        self
    }
    pub fn get_can_delete_messages(&'a self) -> &'a Option<bool> {
        &self.can_delete_messages
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can manage video chats"]
    pub fn can_manage_video_chats(mut self, can_manage_video_chats: bool) -> Self {
        self.can_manage_video_chats = Some(can_manage_video_chats);
        self
    }
    pub fn get_can_manage_video_chats(&'a self) -> &'a Option<bool> {
        &self.can_manage_video_chats
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can restrict, ban or unban chat members, or access supergroup statistics"]
    pub fn can_restrict_members(mut self, can_restrict_members: bool) -> Self {
        self.can_restrict_members = Some(can_restrict_members);
        self
    }
    pub fn get_can_restrict_members(&'a self) -> &'a Option<bool> {
        &self.can_restrict_members
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)"]
    pub fn can_promote_members(mut self, can_promote_members: bool) -> Self {
        self.can_promote_members = Some(can_promote_members);
        self
    }
    pub fn get_can_promote_members(&'a self) -> &'a Option<bool> {
        &self.can_promote_members
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can change chat title, photo and other settings"]
    pub fn can_change_info(mut self, can_change_info: bool) -> Self {
        self.can_change_info = Some(can_change_info);
        self
    }
    pub fn get_can_change_info(&'a self) -> &'a Option<bool> {
        &self.can_change_info
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can invite new users to the chat"]
    pub fn can_invite_users(mut self, can_invite_users: bool) -> Self {
        self.can_invite_users = Some(can_invite_users);
        self
    }
    pub fn get_can_invite_users(&'a self) -> &'a Option<bool> {
        &self.can_invite_users
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can post messages in the channel, or access channel statistics; channels only"]
    pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }
    pub fn get_can_post_messages(&'a self) -> &'a Option<bool> {
        &self.can_post_messages
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can edit messages of other users and can pin messages; channels only"]
    pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }
    pub fn get_can_edit_messages(&'a self) -> &'a Option<bool> {
        &self.can_edit_messages
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can pin messages, supergroups only"]
    pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }
    pub fn get_can_pin_messages(&'a self) -> &'a Option<bool> {
        &self.can_pin_messages
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can post stories in the channel; channels only"]
    pub fn can_post_stories(mut self, can_post_stories: bool) -> Self {
        self.can_post_stories = Some(can_post_stories);
        self
    }
    pub fn get_can_post_stories(&'a self) -> &'a Option<bool> {
        &self.can_post_stories
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can edit stories posted by other users; channels only"]
    pub fn can_edit_stories(mut self, can_edit_stories: bool) -> Self {
        self.can_edit_stories = Some(can_edit_stories);
        self
    }
    pub fn get_can_edit_stories(&'a self) -> &'a Option<bool> {
        &self.can_edit_stories
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the administrator can delete stories posted by other users; channels only"]
    pub fn can_delete_stories(mut self, can_delete_stories: bool) -> Self {
        self.can_delete_stories = Some(can_delete_stories);
        self
    }
    pub fn get_can_delete_stories(&'a self) -> &'a Option<bool> {
        &self.can_delete_stories
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the user is allowed to create, rename, close, and reopen forum topics, supergroups only"]
    pub fn can_manage_topics(mut self, can_manage_topics: bool) -> Self {
        self.can_manage_topics = Some(can_manage_topics);
        self
    }
    pub fn get_can_manage_topics(&'a self) -> &'a Option<bool> {
        &self.can_manage_topics
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .promote_chat_member(
                self.chat_id,
                self.user_id,
                self.is_anonymous,
                self.can_manage_chat,
                self.can_delete_messages,
                self.can_manage_video_chats,
                self.can_restrict_members,
                self.can_promote_members,
                self.can_change_info,
                self.can_invite_users,
                self.can_post_messages,
                self.can_edit_messages,
                self.can_pin_messages,
                self.can_post_stories,
                self.can_edit_stories,
                self.can_delete_stories,
                self.can_manage_topics,
            )
            .await
    }
}
pub struct CallDeleteMyCommands<'a> {
    bot: &'a Bot,
    scope: Option<&'a BotCommandScope>,
    language_code: Option<&'a str>,
}
impl<'a> CallDeleteMyCommands<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault."]
    pub fn scope(mut self, scope: &'a BotCommandScope) -> Self {
        self.scope = Some(scope);
        self
    }
    pub fn get_scope(&'a self) -> &'a Option<&'a BotCommandScope> {
        &self.scope
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands"]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .delete_my_commands(self.scope, self.language_code)
            .await
    }
}
pub struct CallExportChatInviteLink<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallExportChatInviteLink<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<String> {
        self.bot.export_chat_invite_link(self.chat_id).await
    }
}
pub struct CallGetChatMenuButton<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
}
impl<'a> CallGetChatMenuButton<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target private chat. If not specified, default bot's menu button will be returned"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<MenuButton> {
        self.bot.get_chat_menu_button(self.chat_id).await
    }
}
pub struct CallEditMessageLiveLocation<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    latitude: ::ordered_float::OrderedFloat<f64>,
    longitude: ::ordered_float::OrderedFloat<f64>,
    horizontal_accuracy: Option<::ordered_float::OrderedFloat<f64>>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallEditMessageLiveLocation<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the message to edit"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Latitude of new location"]
    pub fn latitude(mut self, latitude: ::ordered_float::OrderedFloat<f64>) -> Self {
        self.latitude = latitude;
        self
    }
    pub fn get_latitude(&'a self) -> &'a ::ordered_float::OrderedFloat<f64> {
        &self.latitude
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Longitude of new location"]
    pub fn longitude(mut self, longitude: ::ordered_float::OrderedFloat<f64>) -> Self {
        self.longitude = longitude;
        self
    }
    pub fn get_longitude(&'a self) -> &'a ::ordered_float::OrderedFloat<f64> {
        &self.longitude
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The radius of uncertainty for the location, measured in meters; 0-1500"]
    pub fn horizontal_accuracy(
        mut self,
        horizontal_accuracy: ::ordered_float::OrderedFloat<f64>,
    ) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }
    pub fn get_horizontal_accuracy(&'a self) -> &'a Option<::ordered_float::OrderedFloat<f64>> {
        &self.horizontal_accuracy
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified."]
    pub fn heading(mut self, heading: i64) -> Self {
        self.heading = Some(heading);
        self
    }
    pub fn get_heading(&'a self) -> &'a Option<i64> {
        &self.heading
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified."]
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: i64) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }
    pub fn get_proximity_alert_radius(&'a self) -> &'a Option<i64> {
        &self.proximity_alert_radius
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for a new inline keyboard."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<MessageBool> {
        self.bot
            .edit_message_live_location(
                self.chat_id,
                self.message_id,
                self.inline_message_id,
                self.latitude,
                self.longitude,
                self.horizontal_accuracy,
                self.heading,
                self.proximity_alert_radius,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallDeleteMessage<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_id: i64,
}
impl<'a> CallDeleteMessage<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Identifier of the message to delete"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }
    pub fn get_message_id(&'a self) -> &'a i64 {
        &self.message_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.delete_message(self.chat_id, self.message_id).await
    }
}
pub struct CallSetStickerMaskPosition<'a> {
    bot: &'a Bot,
    sticker: &'a str,
    mask_position: Option<&'a MaskPosition>,
}
impl<'a> CallSetStickerMaskPosition<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "File identifier of the sticker"]
    pub fn sticker(mut self, sticker: &'a str) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a &'a str {
        &self.sticker
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position."]
    pub fn mask_position(mut self, mask_position: &'a MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
    pub fn get_mask_position(&'a self) -> &'a Option<&'a MaskPosition> {
        &self.mask_position
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_sticker_mask_position(&self.sticker, self.mask_position)
            .await
    }
}
pub struct CallSetChatTitle<'a> {
    bot: &'a Bot,
    chat_id: i64,
    title: &'a str,
}
impl<'a> CallSetChatTitle<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New chat title, 1-128 characters"]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
    pub fn get_title(&'a self) -> &'a &'a str {
        &self.title
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.set_chat_title(self.chat_id, &self.title).await
    }
}
pub struct CallSetPassportDataErrors<'a> {
    bot: &'a Bot,
    user_id: i64,
    errors: &'a Vec<PassportElementError>,
}
impl<'a> CallSetPassportDataErrors<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "User identifier"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized array describing the errors"]
    pub fn errors(mut self, errors: &'a Vec<PassportElementError>) -> Self {
        self.errors = errors;
        self
    }
    pub fn get_errors(&'a self) -> &'a &'a Vec<PassportElementError> {
        &self.errors
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_passport_data_errors(self.user_id, &self.errors)
            .await
    }
}
pub struct CallClose<'a> {
    bot: &'a Bot,
}
impl<'a> CallClose<'a> {
    pub async fn build(self) -> BotResult<bool> {
        self.bot.close().await
    }
}
pub struct CallSendSticker<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    sticker: FileData,
    emoji: Option<&'a str>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendSticker<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP or .TGS sticker using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files. Video stickers can only be sent by a file_id. Animated stickers can't be sent via an HTTP URL."]
    pub fn sticker(mut self, sticker: FileData) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a FileData {
        &self.sticker
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Emoji associated with the sticker; only for just uploaded stickers"]
    pub fn emoji(mut self, emoji: &'a str) -> Self {
        self.emoji = Some(emoji);
        self
    }
    pub fn get_emoji(&'a self) -> &'a Option<&'a str> {
        &self.emoji
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_sticker(
                self.chat_id,
                self.message_thread_id,
                self.sticker,
                self.emoji,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallDeleteWebhook<'a> {
    bot: &'a Bot,
    drop_pending_updates: Option<bool>,
}
impl<'a> CallDeleteWebhook<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True to drop all pending updates"]
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
    pub fn get_drop_pending_updates(&'a self) -> &'a Option<bool> {
        &self.drop_pending_updates
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.delete_webhook(self.drop_pending_updates).await
    }
}
pub struct CallSetMyDefaultAdministratorRights<'a> {
    bot: &'a Bot,
    rights: Option<&'a ChatAdministratorRights>,
    for_channels: Option<bool>,
}
impl<'a> CallSetMyDefaultAdministratorRights<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared."]
    pub fn rights(mut self, rights: &'a ChatAdministratorRights) -> Self {
        self.rights = Some(rights);
        self
    }
    pub fn get_rights(&'a self) -> &'a Option<&'a ChatAdministratorRights> {
        &self.rights
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed."]
    pub fn for_channels(mut self, for_channels: bool) -> Self {
        self.for_channels = Some(for_channels);
        self
    }
    pub fn get_for_channels(&'a self) -> &'a Option<bool> {
        &self.for_channels
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_my_default_administrator_rights(self.rights, self.for_channels)
            .await
    }
}
pub struct CallUnpinAllGeneralForumTopicMessages<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallUnpinAllGeneralForumTopicMessages<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .unpin_all_general_forum_topic_messages(self.chat_id)
            .await
    }
}
pub struct CallEditChatInviteLink<'a> {
    bot: &'a Bot,
    chat_id: i64,
    invite_link: &'a str,
    name: Option<&'a str>,
    expire_date: Option<i64>,
    member_limit: Option<i64>,
    creates_join_request: Option<bool>,
}
impl<'a> CallEditChatInviteLink<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The invite link to edit"]
    pub fn invite_link(mut self, invite_link: &'a str) -> Self {
        self.invite_link = invite_link;
        self
    }
    pub fn get_invite_link(&'a self) -> &'a &'a str {
        &self.invite_link
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Invite link name; 0-32 characters"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
    pub fn get_name(&'a self) -> &'a Option<&'a str> {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Point in time (Unix timestamp) when the link will expire"]
    pub fn expire_date(mut self, expire_date: i64) -> Self {
        self.expire_date = Some(expire_date);
        self
    }
    pub fn get_expire_date(&'a self) -> &'a Option<i64> {
        &self.expire_date
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999"]
    pub fn member_limit(mut self, member_limit: i64) -> Self {
        self.member_limit = Some(member_limit);
        self
    }
    pub fn get_member_limit(&'a self) -> &'a Option<i64> {
        &self.member_limit
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified"]
    pub fn creates_join_request(mut self, creates_join_request: bool) -> Self {
        self.creates_join_request = Some(creates_join_request);
        self
    }
    pub fn get_creates_join_request(&'a self) -> &'a Option<bool> {
        &self.creates_join_request
    }
    pub async fn build(self) -> BotResult<ChatInviteLink> {
        self.bot
            .edit_chat_invite_link(
                self.chat_id,
                &self.invite_link,
                self.name,
                self.expire_date,
                self.member_limit,
                self.creates_join_request,
            )
            .await
    }
}
pub struct CallEditMessageCaption<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallEditMessageCaption<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the message to edit"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New caption of the message, 0-1024 characters after entities parsing"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the message caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for an inline keyboard."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<MessageBool> {
        self.bot
            .edit_message_caption(
                self.chat_id,
                self.message_id,
                self.inline_message_id,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallDeleteStickerFromSet<'a> {
    bot: &'a Bot,
    sticker: &'a str,
}
impl<'a> CallDeleteStickerFromSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "File identifier of the sticker"]
    pub fn sticker(mut self, sticker: &'a str) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a &'a str {
        &self.sticker
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.delete_sticker_from_set(&self.sticker).await
    }
}
pub struct CallSendAnimation<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    animation: FileData,
    duration: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    thumbnail: Option<FileData>,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendAnimation<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn animation(mut self, animation: FileData) -> Self {
        self.animation = animation;
        self
    }
    pub fn get_animation(&'a self) -> &'a FileData {
        &self.animation
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Duration of sent animation in seconds"]
    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }
    pub fn get_duration(&'a self) -> &'a Option<i64> {
        &self.duration
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Animation width"]
    pub fn width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }
    pub fn get_width(&'a self) -> &'a Option<i64> {
        &self.width
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Animation height"]
    pub fn height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }
    pub fn get_height(&'a self) -> &'a Option<i64> {
        &self.height
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass \"attach://<file_attach_name>\" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn thumbnail(mut self, thumbnail: FileData) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }
    pub fn get_thumbnail(&'a self) -> &'a Option<FileData> {
        &self.thumbnail
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Animation caption (may also be used when resending animation by file_id), 0-1024 characters after entities parsing"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the animation caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the animation needs to be covered with a spoiler animation"]
    pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }
    pub fn get_has_spoiler(&'a self) -> &'a Option<bool> {
        &self.has_spoiler
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_animation(
                self.chat_id,
                self.message_thread_id,
                self.animation,
                self.duration,
                self.width,
                self.height,
                self.thumbnail,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.has_spoiler,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallSetChatStickerSet<'a> {
    bot: &'a Bot,
    chat_id: i64,
    sticker_set_name: &'a str,
}
impl<'a> CallSetChatStickerSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Name of the sticker set to be set as the group sticker set"]
    pub fn sticker_set_name(mut self, sticker_set_name: &'a str) -> Self {
        self.sticker_set_name = sticker_set_name;
        self
    }
    pub fn get_sticker_set_name(&'a self) -> &'a &'a str {
        &self.sticker_set_name
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_chat_sticker_set(self.chat_id, &self.sticker_set_name)
            .await
    }
}
pub struct CallSendChatAction<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    action: &'a str,
}
impl<'a> CallSendChatAction<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread; supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_voice or upload_voice for voice notes, upload_document for general files, choose_sticker for stickers, find_location for location data, record_video_note or upload_video_note for video notes."]
    pub fn action(mut self, action: &'a str) -> Self {
        self.action = action;
        self
    }
    pub fn get_action(&'a self) -> &'a &'a str {
        &self.action
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .send_chat_action(self.chat_id, self.message_thread_id, &self.action)
            .await
    }
}
pub struct CallGetFile<'a> {
    bot: &'a Bot,
    file_id: &'a str,
}
impl<'a> CallGetFile<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "File identifier to get information about"]
    pub fn file_id(mut self, file_id: &'a str) -> Self {
        self.file_id = file_id;
        self
    }
    pub fn get_file_id(&'a self) -> &'a &'a str {
        &self.file_id
    }
    pub async fn build(self) -> BotResult<File> {
        self.bot.get_file(&self.file_id).await
    }
}
pub struct CallAddStickerToSet<'a> {
    bot: &'a Bot,
    user_id: i64,
    name: &'a str,
    sticker: &'a InputSticker,
}
impl<'a> CallAddStickerToSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "User identifier of sticker set owner"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker set name"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed."]
    pub fn sticker(mut self, sticker: &'a InputSticker) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a &'a InputSticker {
        &self.sticker
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .add_sticker_to_set(self.user_id, &self.name, &self.sticker)
            .await
    }
}
pub struct CallGetMyName<'a> {
    bot: &'a Bot,
    language_code: Option<&'a str>,
}
impl<'a> CallGetMyName<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code or an empty string"]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<BotName> {
        self.bot.get_my_name(self.language_code).await
    }
}
pub struct CallSetStickerSetTitle<'a> {
    bot: &'a Bot,
    name: &'a str,
    title: &'a str,
}
impl<'a> CallSetStickerSetTitle<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker set name"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker set title, 1-64 characters"]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
    pub fn get_title(&'a self) -> &'a &'a str {
        &self.title
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_sticker_set_title(&self.name, &self.title)
            .await
    }
}
pub struct CallCloseForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: i64,
}
impl<'a> CallCloseForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread of the forum topic"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = message_thread_id;
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a i64 {
        &self.message_thread_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .close_forum_topic(self.chat_id, self.message_thread_id)
            .await
    }
}
pub struct CallGetMyCommands<'a> {
    bot: &'a Bot,
    scope: Option<&'a BotCommandScope>,
    language_code: Option<&'a str>,
}
impl<'a> CallGetMyCommands<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object, describing scope of users. Defaults to BotCommandScopeDefault."]
    pub fn scope(mut self, scope: &'a BotCommandScope) -> Self {
        self.scope = Some(scope);
        self
    }
    pub fn get_scope(&'a self) -> &'a Option<&'a BotCommandScope> {
        &self.scope
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code or an empty string"]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<Vec<BotCommand>> {
        self.bot
            .get_my_commands(self.scope, self.language_code)
            .await
    }
}
pub struct CallCopyMessage<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    from_chat_id: i64,
    message_id: i64,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallCopyMessage<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)"]
    pub fn from_chat_id(mut self, from_chat_id: i64) -> Self {
        self.from_chat_id = from_chat_id;
        self
    }
    pub fn get_from_chat_id(&'a self) -> &'a i64 {
        &self.from_chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Message identifier in the chat specified in from_chat_id"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }
    pub fn get_message_id(&'a self) -> &'a i64 {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the new caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<MessageId> {
        self.bot
            .copy_message(
                self.chat_id,
                self.message_thread_id,
                self.from_chat_id,
                self.message_id,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallUnpinAllChatMessages<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallUnpinAllChatMessages<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.unpin_all_chat_messages(self.chat_id).await
    }
}
pub struct CallGetChatMemberCount<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallGetChatMemberCount<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<i64> {
        self.bot.get_chat_member_count(self.chat_id).await
    }
}
pub struct CallAnswerPreCheckoutQuery<'a> {
    bot: &'a Bot,
    pre_checkout_query_id: &'a str,
    ok: bool,
    error_message: Option<&'a str>,
}
impl<'a> CallAnswerPreCheckoutQuery<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the query to be answered"]
    pub fn pre_checkout_query_id(mut self, pre_checkout_query_id: &'a str) -> Self {
        self.pre_checkout_query_id = pre_checkout_query_id;
        self
    }
    pub fn get_pre_checkout_query_id(&'a self) -> &'a &'a str {
        &self.pre_checkout_query_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Specify True if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use False if there are any problems."]
    pub fn ok(mut self, ok: bool) -> Self {
        self.ok = ok;
        self
    }
    pub fn get_ok(&'a self) -> &'a bool {
        &self.ok
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. \"Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!\"). Telegram will display this message to the user."]
    pub fn error_message(mut self, error_message: &'a str) -> Self {
        self.error_message = Some(error_message);
        self
    }
    pub fn get_error_message(&'a self) -> &'a Option<&'a str> {
        &self.error_message
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .answer_pre_checkout_query(&self.pre_checkout_query_id, self.ok, self.error_message)
            .await
    }
}
pub struct CallCreateChatInviteLink<'a> {
    bot: &'a Bot,
    chat_id: i64,
    name: Option<&'a str>,
    expire_date: Option<i64>,
    member_limit: Option<i64>,
    creates_join_request: Option<bool>,
}
impl<'a> CallCreateChatInviteLink<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Invite link name; 0-32 characters"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
    pub fn get_name(&'a self) -> &'a Option<&'a str> {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Point in time (Unix timestamp) when the link will expire"]
    pub fn expire_date(mut self, expire_date: i64) -> Self {
        self.expire_date = Some(expire_date);
        self
    }
    pub fn get_expire_date(&'a self) -> &'a Option<i64> {
        &self.expire_date
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999"]
    pub fn member_limit(mut self, member_limit: i64) -> Self {
        self.member_limit = Some(member_limit);
        self
    }
    pub fn get_member_limit(&'a self) -> &'a Option<i64> {
        &self.member_limit
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified"]
    pub fn creates_join_request(mut self, creates_join_request: bool) -> Self {
        self.creates_join_request = Some(creates_join_request);
        self
    }
    pub fn get_creates_join_request(&'a self) -> &'a Option<bool> {
        &self.creates_join_request
    }
    pub async fn build(self) -> BotResult<ChatInviteLink> {
        self.bot
            .create_chat_invite_link(
                self.chat_id,
                self.name,
                self.expire_date,
                self.member_limit,
                self.creates_join_request,
            )
            .await
    }
}
pub struct CallEditForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: i64,
    name: Option<&'a str>,
    icon_custom_emoji_id: Option<&'a str>,
}
impl<'a> CallEditForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread of the forum topic"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = message_thread_id;
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a i64 {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
    pub fn get_name(&'a self) -> &'a Option<&'a str> {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New unique identifier of the custom emoji shown as the topic icon. Use getForumTopicIconStickers to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept"]
    pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: &'a str) -> Self {
        self.icon_custom_emoji_id = Some(icon_custom_emoji_id);
        self
    }
    pub fn get_icon_custom_emoji_id(&'a self) -> &'a Option<&'a str> {
        &self.icon_custom_emoji_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .edit_forum_topic(
                self.chat_id,
                self.message_thread_id,
                self.name,
                self.icon_custom_emoji_id,
            )
            .await
    }
}
pub struct CallSetWebhook<'a> {
    bot: &'a Bot,
    url: &'a str,
    certificate: Option<FileData>,
    ip_address: Option<&'a str>,
    max_connections: Option<i64>,
    allowed_updates: Option<&'a Vec<String>>,
    drop_pending_updates: Option<bool>,
    secret_token: Option<&'a str>,
}
impl<'a> CallSetWebhook<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "HTTPS URL to send updates to. Use an empty string to remove webhook integration"]
    pub fn url(mut self, url: &'a str) -> Self {
        self.url = url;
        self
    }
    pub fn get_url(&'a self) -> &'a &'a str {
        &self.url
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details."]
    pub fn certificate(mut self, certificate: FileData) -> Self {
        self.certificate = Some(certificate);
        self
    }
    pub fn get_certificate(&'a self) -> &'a Option<FileData> {
        &self.certificate
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS"]
    pub fn ip_address(mut self, ip_address: &'a str) -> Self {
        self.ip_address = Some(ip_address);
        self
    }
    pub fn get_ip_address(&'a self) -> &'a Option<&'a str> {
        &self.ip_address
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput."]
    pub fn max_connections(mut self, max_connections: i64) -> Self {
        self.max_connections = Some(max_connections);
        self
    }
    pub fn get_max_connections(&'a self) -> &'a Option<i64> {
        &self.max_connections
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of the update types you want your bot to receive. For example, specify [\"message\", \"edited_channel_post\", \"callback_query\"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time."]
    pub fn allowed_updates(mut self, allowed_updates: &'a Vec<String>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }
    pub fn get_allowed_updates(&'a self) -> &'a Option<&'a Vec<String>> {
        &self.allowed_updates
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True to drop all pending updates"]
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
    pub fn get_drop_pending_updates(&'a self) -> &'a Option<bool> {
        &self.drop_pending_updates
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A secret token to be sent in a header \"X-Telegram-Bot-Api-Secret-Token\" in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, _ and - are allowed. The header is useful to ensure that the request comes from a webhook set by you."]
    pub fn secret_token(mut self, secret_token: &'a str) -> Self {
        self.secret_token = Some(secret_token);
        self
    }
    pub fn get_secret_token(&'a self) -> &'a Option<&'a str> {
        &self.secret_token
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_webhook(
                &self.url,
                self.certificate,
                self.ip_address,
                self.max_connections,
                self.allowed_updates,
                self.drop_pending_updates,
                self.secret_token,
            )
            .await
    }
}
pub struct CallLogOut<'a> {
    bot: &'a Bot,
}
impl<'a> CallLogOut<'a> {
    pub async fn build(self) -> BotResult<bool> {
        self.bot.log_out().await
    }
}
pub struct CallRevokeChatInviteLink<'a> {
    bot: &'a Bot,
    chat_id: i64,
    invite_link: &'a str,
}
impl<'a> CallRevokeChatInviteLink<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The invite link to revoke"]
    pub fn invite_link(mut self, invite_link: &'a str) -> Self {
        self.invite_link = invite_link;
        self
    }
    pub fn get_invite_link(&'a self) -> &'a &'a str {
        &self.invite_link
    }
    pub async fn build(self) -> BotResult<ChatInviteLink> {
        self.bot
            .revoke_chat_invite_link(self.chat_id, &self.invite_link)
            .await
    }
}
pub struct CallReopenForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: i64,
}
impl<'a> CallReopenForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread of the forum topic"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = message_thread_id;
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a i64 {
        &self.message_thread_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .reopen_forum_topic(self.chat_id, self.message_thread_id)
            .await
    }
}
pub struct CallGetMyShortDescription<'a> {
    bot: &'a Bot,
    language_code: Option<&'a str>,
}
impl<'a> CallGetMyShortDescription<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code or an empty string"]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<BotShortDescription> {
        self.bot.get_my_short_description(self.language_code).await
    }
}
pub struct CallGetMyDefaultAdministratorRights<'a> {
    bot: &'a Bot,
    for_channels: Option<bool>,
}
impl<'a> CallGetMyDefaultAdministratorRights<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned."]
    pub fn for_channels(mut self, for_channels: bool) -> Self {
        self.for_channels = Some(for_channels);
        self
    }
    pub fn get_for_channels(&'a self) -> &'a Option<bool> {
        &self.for_channels
    }
    pub async fn build(self) -> BotResult<ChatAdministratorRights> {
        self.bot
            .get_my_default_administrator_rights(self.for_channels)
            .await
    }
}
pub struct CallSetStickerKeywords<'a> {
    bot: &'a Bot,
    sticker: &'a str,
    keywords: Option<&'a Vec<String>>,
}
impl<'a> CallSetStickerKeywords<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "File identifier of the sticker"]
    pub fn sticker(mut self, sticker: &'a str) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn get_sticker(&'a self) -> &'a &'a str {
        &self.sticker
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters"]
    pub fn keywords(mut self, keywords: &'a Vec<String>) -> Self {
        self.keywords = Some(keywords);
        self
    }
    pub fn get_keywords(&'a self) -> &'a Option<&'a Vec<String>> {
        &self.keywords
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_sticker_keywords(&self.sticker, self.keywords)
            .await
    }
}
pub struct CallSetChatMenuButton<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
    menu_button: Option<&'a MenuButton>,
}
impl<'a> CallSetChatMenuButton<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target private chat. If not specified, default bot's menu button will be changed"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for the bot's new menu button. Defaults to MenuButtonDefault"]
    pub fn menu_button(mut self, menu_button: &'a MenuButton) -> Self {
        self.menu_button = Some(menu_button);
        self
    }
    pub fn get_menu_button(&'a self) -> &'a Option<&'a MenuButton> {
        &self.menu_button
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_chat_menu_button(self.chat_id, self.menu_button)
            .await
    }
}
pub struct CallEditMessageMedia<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    media: &'a InputMedia,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallEditMessageMedia<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the message to edit"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for a new media content of the message"]
    pub fn media(mut self, media: &'a InputMedia) -> Self {
        self.media = media;
        self
    }
    pub fn get_media(&'a self) -> &'a &'a InputMedia {
        &self.media
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for a new inline keyboard."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<MessageBool> {
        self.bot
            .edit_message_media(
                self.chat_id,
                self.message_id,
                self.inline_message_id,
                &self.media,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallReopenGeneralForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallReopenGeneralForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.reopen_general_forum_topic(self.chat_id).await
    }
}
pub struct CallGetGameHighScores<'a> {
    bot: &'a Bot,
    user_id: i64,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
}
impl<'a> CallGetGameHighScores<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Target user id"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the sent message"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    pub async fn build(self) -> BotResult<Vec<GameHighScore>> {
        self.bot
            .get_game_high_scores(
                self.user_id,
                self.chat_id,
                self.message_id,
                self.inline_message_id,
            )
            .await
    }
}
pub struct CallCreateForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
    name: &'a str,
    icon_color: Option<i64>,
    icon_custom_emoji_id: Option<&'a str>,
}
impl<'a> CallCreateForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Topic name, 1-128 characters"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)"]
    pub fn icon_color(mut self, icon_color: i64) -> Self {
        self.icon_color = Some(icon_color);
        self
    }
    pub fn get_icon_color(&'a self) -> &'a Option<i64> {
        &self.icon_color
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the custom emoji shown as the topic icon. Use getForumTopicIconStickers to get all allowed custom emoji identifiers."]
    pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: &'a str) -> Self {
        self.icon_custom_emoji_id = Some(icon_custom_emoji_id);
        self
    }
    pub fn get_icon_custom_emoji_id(&'a self) -> &'a Option<&'a str> {
        &self.icon_custom_emoji_id
    }
    pub async fn build(self) -> BotResult<ForumTopic> {
        self.bot
            .create_forum_topic(
                self.chat_id,
                &self.name,
                self.icon_color,
                self.icon_custom_emoji_id,
            )
            .await
    }
}
pub struct CallSendInvoice<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    title: &'a str,
    description: &'a str,
    payload: &'a str,
    provider_token: &'a str,
    currency: &'a str,
    prices: &'a Vec<LabeledPrice>,
    max_tip_amount: Option<i64>,
    suggested_tip_amounts: Option<Vec<i64>>,
    start_parameter: Option<&'a str>,
    provider_data: Option<&'a str>,
    photo_url: Option<&'a str>,
    photo_size: Option<i64>,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallSendInvoice<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Product name, 1-32 characters"]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
    pub fn get_title(&'a self) -> &'a &'a str {
        &self.title
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Product description, 1-255 characters"]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }
    pub fn get_description(&'a self) -> &'a &'a str {
        &self.description
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes."]
    pub fn payload(mut self, payload: &'a str) -> Self {
        self.payload = payload;
        self
    }
    pub fn get_payload(&'a self) -> &'a &'a str {
        &self.payload
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Payment provider token, obtained via @BotFather"]
    pub fn provider_token(mut self, provider_token: &'a str) -> Self {
        self.provider_token = provider_token;
        self
    }
    pub fn get_provider_token(&'a self) -> &'a &'a str {
        &self.provider_token
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Three-letter ISO 4217 currency code, see more on currencies"]
    pub fn currency(mut self, currency: &'a str) -> Self {
        self.currency = currency;
        self
    }
    pub fn get_currency(&'a self) -> &'a &'a str {
        &self.currency
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)"]
    pub fn prices(mut self, prices: &'a Vec<LabeledPrice>) -> Self {
        self.prices = prices;
        self
    }
    pub fn get_prices(&'a self) -> &'a &'a Vec<LabeledPrice> {
        &self.prices
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0"]
    pub fn max_tip_amount(mut self, max_tip_amount: i64) -> Self {
        self.max_tip_amount = Some(max_tip_amount);
        self
    }
    pub fn get_max_tip_amount(&'a self) -> &'a Option<i64> {
        &self.max_tip_amount
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount."]
    pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(suggested_tip_amounts);
        self
    }
    pub fn get_suggested_tip_amounts(&'a self) -> &'a Option<Vec<i64>> {
        &self.suggested_tip_amounts
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter"]
    pub fn start_parameter(mut self, start_parameter: &'a str) -> Self {
        self.start_parameter = Some(start_parameter);
        self
    }
    pub fn get_start_parameter(&'a self) -> &'a Option<&'a str> {
        &self.start_parameter
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider."]
    pub fn provider_data(mut self, provider_data: &'a str) -> Self {
        self.provider_data = Some(provider_data);
        self
    }
    pub fn get_provider_data(&'a self) -> &'a Option<&'a str> {
        &self.provider_data
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for."]
    pub fn photo_url(mut self, photo_url: &'a str) -> Self {
        self.photo_url = Some(photo_url);
        self
    }
    pub fn get_photo_url(&'a self) -> &'a Option<&'a str> {
        &self.photo_url
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo size in bytes"]
    pub fn photo_size(mut self, photo_size: i64) -> Self {
        self.photo_size = Some(photo_size);
        self
    }
    pub fn get_photo_size(&'a self) -> &'a Option<i64> {
        &self.photo_size
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo width"]
    pub fn photo_width(mut self, photo_width: i64) -> Self {
        self.photo_width = Some(photo_width);
        self
    }
    pub fn get_photo_width(&'a self) -> &'a Option<i64> {
        &self.photo_width
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo height"]
    pub fn photo_height(mut self, photo_height: i64) -> Self {
        self.photo_height = Some(photo_height);
        self
    }
    pub fn get_photo_height(&'a self) -> &'a Option<i64> {
        &self.photo_height
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's full name to complete the order"]
    pub fn need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);
        self
    }
    pub fn get_need_name(&'a self) -> &'a Option<bool> {
        &self.need_name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's phone number to complete the order"]
    pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }
    pub fn get_need_phone_number(&'a self) -> &'a Option<bool> {
        &self.need_phone_number
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's email address to complete the order"]
    pub fn need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);
        self
    }
    pub fn get_need_email(&'a self) -> &'a Option<bool> {
        &self.need_email
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's shipping address to complete the order"]
    pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }
    pub fn get_need_shipping_address(&'a self) -> &'a Option<bool> {
        &self.need_shipping_address
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the user's phone number should be sent to provider"]
    pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }
    pub fn get_send_phone_number_to_provider(&'a self) -> &'a Option<bool> {
        &self.send_phone_number_to_provider
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the user's email address should be sent to provider"]
    pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }
    pub fn get_send_email_to_provider(&'a self) -> &'a Option<bool> {
        &self.send_email_to_provider
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the final price depends on the shipping method"]
    pub fn is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }
    pub fn get_is_flexible(&'a self) -> &'a Option<bool> {
        &self.is_flexible
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_invoice(
                self.chat_id,
                self.message_thread_id,
                &self.title,
                &self.description,
                &self.payload,
                &self.provider_token,
                &self.currency,
                &self.prices,
                self.max_tip_amount,
                self.suggested_tip_amounts,
                self.start_parameter,
                self.provider_data,
                self.photo_url,
                self.photo_size,
                self.photo_width,
                self.photo_height,
                self.need_name,
                self.need_phone_number,
                self.need_email,
                self.need_shipping_address,
                self.send_phone_number_to_provider,
                self.send_email_to_provider,
                self.is_flexible,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallLeaveChat<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallLeaveChat<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.leave_chat(self.chat_id).await
    }
}
pub struct CallDeleteChatStickerSet<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallDeleteChatStickerSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.delete_chat_sticker_set(self.chat_id).await
    }
}
pub struct CallGetStickerSet<'a> {
    bot: &'a Bot,
    name: &'a str,
}
impl<'a> CallGetStickerSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Name of the sticker set"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    pub async fn build(self) -> BotResult<StickerSet> {
        self.bot.get_sticker_set(&self.name).await
    }
}
pub struct CallGetMyDescription<'a> {
    bot: &'a Bot,
    language_code: Option<&'a str>,
}
impl<'a> CallGetMyDescription<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code or an empty string"]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<BotDescription> {
        self.bot.get_my_description(self.language_code).await
    }
}
pub struct CallSendPhoto<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    photo: FileData,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendPhoto<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn photo(mut self, photo: FileData) -> Self {
        self.photo = photo;
        self
    }
    pub fn get_photo(&'a self) -> &'a FileData {
        &self.photo
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo caption (may also be used when resending photos by file_id), 0-1024 characters after entities parsing"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the photo caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the photo needs to be covered with a spoiler animation"]
    pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }
    pub fn get_has_spoiler(&'a self) -> &'a Option<bool> {
        &self.has_spoiler
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_photo(
                self.chat_id,
                self.message_thread_id,
                self.photo,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.has_spoiler,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallCloseGeneralForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallCloseGeneralForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.close_general_forum_topic(self.chat_id).await
    }
}
pub struct CallSetChatDescription<'a> {
    bot: &'a Bot,
    chat_id: i64,
    description: Option<&'a str>,
}
impl<'a> CallSetChatDescription<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New chat description, 0-255 characters"]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
    pub fn get_description(&'a self) -> &'a Option<&'a str> {
        &self.description
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_chat_description(self.chat_id, self.description)
            .await
    }
}
pub struct CallGetChatAdministrators<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallGetChatAdministrators<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<Vec<ChatMember>> {
        self.bot.get_chat_administrators(self.chat_id).await
    }
}
pub struct CallGetForumTopicIconStickers<'a> {
    bot: &'a Bot,
}
impl<'a> CallGetForumTopicIconStickers<'a> {
    pub async fn build(self) -> BotResult<Vec<Sticker>> {
        self.bot.get_forum_topic_icon_stickers().await
    }
}
pub struct CallUnhideGeneralForumTopic<'a> {
    bot: &'a Bot,
    chat_id: i64,
}
impl<'a> CallUnhideGeneralForumTopic<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.unhide_general_forum_topic(self.chat_id).await
    }
}
pub struct CallStopPoll<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_id: i64,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallStopPoll<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Identifier of the original message with the poll"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }
    pub fn get_message_id(&'a self) -> &'a i64 {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for a new message inline keyboard."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Poll> {
        self.bot
            .stop_poll(self.chat_id, self.message_id, self.reply_markup)
            .await
    }
}
pub struct CallBanChatMember<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
    until_date: Option<i64>,
    revoke_messages: Option<bool>,
}
impl<'a> CallBanChatMember<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only."]
    pub fn until_date(mut self, until_date: i64) -> Self {
        self.until_date = Some(until_date);
        self
    }
    pub fn get_until_date(&'a self) -> &'a Option<i64> {
        &self.until_date
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True to delete all messages from the chat for the user that is being removed. If False, the user will be able to see messages in the group that were sent before the user was removed. Always True for supergroups and channels."]
    pub fn revoke_messages(mut self, revoke_messages: bool) -> Self {
        self.revoke_messages = Some(revoke_messages);
        self
    }
    pub fn get_revoke_messages(&'a self) -> &'a Option<bool> {
        &self.revoke_messages
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .ban_chat_member(
                self.chat_id,
                self.user_id,
                self.until_date,
                self.revoke_messages,
            )
            .await
    }
}
pub struct CallSendVenue<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    latitude: ::ordered_float::OrderedFloat<f64>,
    longitude: ::ordered_float::OrderedFloat<f64>,
    title: &'a str,
    address: &'a str,
    foursquare_id: Option<&'a str>,
    foursquare_type: Option<&'a str>,
    google_place_id: Option<&'a str>,
    google_place_type: Option<&'a str>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendVenue<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Latitude of the venue"]
    pub fn latitude(mut self, latitude: ::ordered_float::OrderedFloat<f64>) -> Self {
        self.latitude = latitude;
        self
    }
    pub fn get_latitude(&'a self) -> &'a ::ordered_float::OrderedFloat<f64> {
        &self.latitude
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Longitude of the venue"]
    pub fn longitude(mut self, longitude: ::ordered_float::OrderedFloat<f64>) -> Self {
        self.longitude = longitude;
        self
    }
    pub fn get_longitude(&'a self) -> &'a ::ordered_float::OrderedFloat<f64> {
        &self.longitude
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Name of the venue"]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
    pub fn get_title(&'a self) -> &'a &'a str {
        &self.title
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Address of the venue"]
    pub fn address(mut self, address: &'a str) -> Self {
        self.address = address;
        self
    }
    pub fn get_address(&'a self) -> &'a &'a str {
        &self.address
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Foursquare identifier of the venue"]
    pub fn foursquare_id(mut self, foursquare_id: &'a str) -> Self {
        self.foursquare_id = Some(foursquare_id);
        self
    }
    pub fn get_foursquare_id(&'a self) -> &'a Option<&'a str> {
        &self.foursquare_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Foursquare type of the venue, if known. (For example, \"arts_entertainment/default\", \"arts_entertainment/aquarium\" or \"food/icecream\".)"]
    pub fn foursquare_type(mut self, foursquare_type: &'a str) -> Self {
        self.foursquare_type = Some(foursquare_type);
        self
    }
    pub fn get_foursquare_type(&'a self) -> &'a Option<&'a str> {
        &self.foursquare_type
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Google Places identifier of the venue"]
    pub fn google_place_id(mut self, google_place_id: &'a str) -> Self {
        self.google_place_id = Some(google_place_id);
        self
    }
    pub fn get_google_place_id(&'a self) -> &'a Option<&'a str> {
        &self.google_place_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Google Places type of the venue. (See supported types.)"]
    pub fn google_place_type(mut self, google_place_type: &'a str) -> Self {
        self.google_place_type = Some(google_place_type);
        self
    }
    pub fn get_google_place_type(&'a self) -> &'a Option<&'a str> {
        &self.google_place_type
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_venue(
                self.chat_id,
                self.message_thread_id,
                self.latitude,
                self.longitude,
                &self.title,
                &self.address,
                self.foursquare_id,
                self.foursquare_type,
                self.google_place_id,
                self.google_place_type,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallDeclineChatJoinRequest<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
}
impl<'a> CallDeclineChatJoinRequest<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .decline_chat_join_request(self.chat_id, self.user_id)
            .await
    }
}
pub struct CallSetChatPhoto<'a> {
    bot: &'a Bot,
    chat_id: i64,
    photo: FileData,
}
impl<'a> CallSetChatPhoto<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New chat photo, uploaded using multipart/form-data"]
    pub fn photo(mut self, photo: FileData) -> Self {
        self.photo = photo;
        self
    }
    pub fn get_photo(&'a self) -> &'a FileData {
        &self.photo
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.set_chat_photo(self.chat_id, self.photo).await
    }
}
pub struct CallCreateInvoiceLink<'a> {
    bot: &'a Bot,
    title: &'a str,
    description: &'a str,
    payload: &'a str,
    provider_token: &'a str,
    currency: &'a str,
    prices: &'a Vec<LabeledPrice>,
    max_tip_amount: Option<i64>,
    suggested_tip_amounts: Option<Vec<i64>>,
    provider_data: Option<&'a str>,
    photo_url: Option<&'a str>,
    photo_size: Option<i64>,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
}
impl<'a> CallCreateInvoiceLink<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Product name, 1-32 characters"]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
    pub fn get_title(&'a self) -> &'a &'a str {
        &self.title
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Product description, 1-255 characters"]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }
    pub fn get_description(&'a self) -> &'a &'a str {
        &self.description
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes."]
    pub fn payload(mut self, payload: &'a str) -> Self {
        self.payload = payload;
        self
    }
    pub fn get_payload(&'a self) -> &'a &'a str {
        &self.payload
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Payment provider token, obtained via BotFather"]
    pub fn provider_token(mut self, provider_token: &'a str) -> Self {
        self.provider_token = provider_token;
        self
    }
    pub fn get_provider_token(&'a self) -> &'a &'a str {
        &self.provider_token
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Three-letter ISO 4217 currency code, see more on currencies"]
    pub fn currency(mut self, currency: &'a str) -> Self {
        self.currency = currency;
        self
    }
    pub fn get_currency(&'a self) -> &'a &'a str {
        &self.currency
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)"]
    pub fn prices(mut self, prices: &'a Vec<LabeledPrice>) -> Self {
        self.prices = prices;
        self
    }
    pub fn get_prices(&'a self) -> &'a &'a Vec<LabeledPrice> {
        &self.prices
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0"]
    pub fn max_tip_amount(mut self, max_tip_amount: i64) -> Self {
        self.max_tip_amount = Some(max_tip_amount);
        self
    }
    pub fn get_max_tip_amount(&'a self) -> &'a Option<i64> {
        &self.max_tip_amount
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount."]
    pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(suggested_tip_amounts);
        self
    }
    pub fn get_suggested_tip_amounts(&'a self) -> &'a Option<Vec<i64>> {
        &self.suggested_tip_amounts
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider."]
    pub fn provider_data(mut self, provider_data: &'a str) -> Self {
        self.provider_data = Some(provider_data);
        self
    }
    pub fn get_provider_data(&'a self) -> &'a Option<&'a str> {
        &self.provider_data
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service."]
    pub fn photo_url(mut self, photo_url: &'a str) -> Self {
        self.photo_url = Some(photo_url);
        self
    }
    pub fn get_photo_url(&'a self) -> &'a Option<&'a str> {
        &self.photo_url
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo size in bytes"]
    pub fn photo_size(mut self, photo_size: i64) -> Self {
        self.photo_size = Some(photo_size);
        self
    }
    pub fn get_photo_size(&'a self) -> &'a Option<i64> {
        &self.photo_size
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo width"]
    pub fn photo_width(mut self, photo_width: i64) -> Self {
        self.photo_width = Some(photo_width);
        self
    }
    pub fn get_photo_width(&'a self) -> &'a Option<i64> {
        &self.photo_width
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Photo height"]
    pub fn photo_height(mut self, photo_height: i64) -> Self {
        self.photo_height = Some(photo_height);
        self
    }
    pub fn get_photo_height(&'a self) -> &'a Option<i64> {
        &self.photo_height
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's full name to complete the order"]
    pub fn need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);
        self
    }
    pub fn get_need_name(&'a self) -> &'a Option<bool> {
        &self.need_name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's phone number to complete the order"]
    pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }
    pub fn get_need_phone_number(&'a self) -> &'a Option<bool> {
        &self.need_phone_number
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's email address to complete the order"]
    pub fn need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);
        self
    }
    pub fn get_need_email(&'a self) -> &'a Option<bool> {
        &self.need_email
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if you require the user's shipping address to complete the order"]
    pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }
    pub fn get_need_shipping_address(&'a self) -> &'a Option<bool> {
        &self.need_shipping_address
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the user's phone number should be sent to the provider"]
    pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }
    pub fn get_send_phone_number_to_provider(&'a self) -> &'a Option<bool> {
        &self.send_phone_number_to_provider
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the user's email address should be sent to the provider"]
    pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }
    pub fn get_send_email_to_provider(&'a self) -> &'a Option<bool> {
        &self.send_email_to_provider
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the final price depends on the shipping method"]
    pub fn is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }
    pub fn get_is_flexible(&'a self) -> &'a Option<bool> {
        &self.is_flexible
    }
    pub async fn build(self) -> BotResult<String> {
        self.bot
            .create_invoice_link(
                &self.title,
                &self.description,
                &self.payload,
                &self.provider_token,
                &self.currency,
                &self.prices,
                self.max_tip_amount,
                self.suggested_tip_amounts,
                self.provider_data,
                self.photo_url,
                self.photo_size,
                self.photo_width,
                self.photo_height,
                self.need_name,
                self.need_phone_number,
                self.need_email,
                self.need_shipping_address,
                self.send_phone_number_to_provider,
                self.send_email_to_provider,
                self.is_flexible,
            )
            .await
    }
}
pub struct CallStopMessageLiveLocation<'a> {
    bot: &'a Bot,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallStopMessageLiveLocation<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the message with live location to stop"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for a new inline keyboard."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<MessageBool> {
        self.bot
            .stop_message_live_location(
                self.chat_id,
                self.message_id,
                self.inline_message_id,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallSendGame<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    game_short_name: &'a str,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a InlineKeyboardMarkup>,
}
impl<'a> CallSendGame<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Short name of the game, serves as the unique identifier for the game. Set up your games via @BotFather."]
    pub fn game_short_name(mut self, game_short_name: &'a str) -> Self {
        self.game_short_name = game_short_name;
        self
    }
    pub fn get_game_short_name(&'a self) -> &'a &'a str {
        &self.game_short_name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game."]
    pub fn reply_markup(mut self, reply_markup: &'a InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a InlineKeyboardMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_game(
                self.chat_id,
                self.message_thread_id,
                &self.game_short_name,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallUnbanChatMember<'a> {
    bot: &'a Bot,
    chat_id: i64,
    user_id: i64,
    only_if_banned: Option<bool>,
}
impl<'a> CallUnbanChatMember<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Do nothing if the user is not banned"]
    pub fn only_if_banned(mut self, only_if_banned: bool) -> Self {
        self.only_if_banned = Some(only_if_banned);
        self
    }
    pub fn get_only_if_banned(&'a self) -> &'a Option<bool> {
        &self.only_if_banned
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .unban_chat_member(self.chat_id, self.user_id, self.only_if_banned)
            .await
    }
}
pub struct CallSendVoice<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    voice: FileData,
    caption: Option<&'a str>,
    parse_mode: Option<&'a str>,
    caption_entities: Option<&'a Vec<MessageEntity>>,
    duration: Option<i64>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendVoice<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn voice(mut self, voice: FileData) -> Self {
        self.voice = voice;
        self
    }
    pub fn get_voice(&'a self) -> &'a FileData {
        &self.voice
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Voice message caption, 0-1024 characters after entities parsing"]
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn get_caption(&'a self) -> &'a Option<&'a str> {
        &self.caption
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the voice message caption. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn caption_entities(mut self, caption_entities: &'a Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn get_caption_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.caption_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Duration of the voice message in seconds"]
    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }
    pub fn get_duration(&'a self) -> &'a Option<i64> {
        &self.duration
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_voice(
                self.chat_id,
                self.message_thread_id,
                self.voice,
                self.caption,
                self.parse_mode,
                self.caption_entities,
                self.duration,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallSendDice<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    emoji: Option<&'a str>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendDice<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Emoji on which the dice throw animation is based. Currently, must be one of \"🎲\", \"🎯\", \"🏀\", \"⚽\", \"🎳\", or \"🎰\". Dice can have values 1-6 for \"🎲\", \"🎯\" and \"🎳\", values 1-5 for \"🏀\" and \"⚽\", and values 1-64 for \"🎰\". Defaults to \"🎲\""]
    pub fn emoji(mut self, emoji: &'a str) -> Self {
        self.emoji = Some(emoji);
        self
    }
    pub fn get_emoji(&'a self) -> &'a Option<&'a str> {
        &self.emoji
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_dice(
                self.chat_id,
                self.message_thread_id,
                self.emoji,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallAnswerCallbackQuery<'a> {
    bot: &'a Bot,
    callback_query_id: &'a str,
    text: Option<&'a str>,
    show_alert: Option<bool>,
    url: Option<&'a str>,
    cache_time: Option<i64>,
}
impl<'a> CallAnswerCallbackQuery<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the query to be answered"]
    pub fn callback_query_id(mut self, callback_query_id: &'a str) -> Self {
        self.callback_query_id = callback_query_id;
        self
    }
    pub fn get_callback_query_id(&'a self) -> &'a &'a str {
        &self.callback_query_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters"]
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }
    pub fn get_text(&'a self) -> &'a Option<&'a str> {
        &self.text
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If True, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false."]
    pub fn show_alert(mut self, show_alert: bool) -> Self {
        self.show_alert = Some(show_alert);
        self
    }
    pub fn get_show_alert(&'a self) -> &'a Option<bool> {
        &self.show_alert
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @BotFather, specify the URL that opens your game - note that this will only work if the query comes from a callback_game button. Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter."]
    pub fn url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }
    pub fn get_url(&'a self) -> &'a Option<&'a str> {
        &self.url
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0."]
    pub fn cache_time(mut self, cache_time: i64) -> Self {
        self.cache_time = Some(cache_time);
        self
    }
    pub fn get_cache_time(&'a self) -> &'a Option<i64> {
        &self.cache_time
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .answer_callback_query(
                &self.callback_query_id,
                self.text,
                self.show_alert,
                self.url,
                self.cache_time,
            )
            .await
    }
}
pub struct CallUnbanChatSenderChat<'a> {
    bot: &'a Bot,
    chat_id: i64,
    sender_chat_id: i64,
}
impl<'a> CallUnbanChatSenderChat<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target sender chat"]
    pub fn sender_chat_id(mut self, sender_chat_id: i64) -> Self {
        self.sender_chat_id = sender_chat_id;
        self
    }
    pub fn get_sender_chat_id(&'a self) -> &'a i64 {
        &self.sender_chat_id
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .unban_chat_sender_chat(self.chat_id, self.sender_chat_id)
            .await
    }
}
pub struct CallSetMyName<'a> {
    bot: &'a Bot,
    name: Option<&'a str>,
    language_code: Option<&'a str>,
}
impl<'a> CallSetMyName<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language."]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
    pub fn get_name(&'a self) -> &'a Option<&'a str> {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name."]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.set_my_name(self.name, self.language_code).await
    }
}
pub struct CallDeleteStickerSet<'a> {
    bot: &'a Bot,
    name: &'a str,
}
impl<'a> CallDeleteStickerSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker set name"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot.delete_sticker_set(&self.name).await
    }
}
pub struct CallSendPoll<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    question: &'a str,
    options: &'a Vec<String>,
    is_anonymous: Option<bool>,
    tg_type: Option<&'a str>,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<i64>,
    explanation: Option<&'a str>,
    explanation_parse_mode: Option<&'a str>,
    explanation_entities: Option<&'a Vec<MessageEntity>>,
    open_period: Option<i64>,
    close_date: Option<i64>,
    is_closed: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendPoll<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Poll question, 1-300 characters"]
    pub fn question(mut self, question: &'a str) -> Self {
        self.question = question;
        self
    }
    pub fn get_question(&'a self) -> &'a &'a str {
        &self.question
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of answer options, 2-10 strings 1-100 characters each"]
    pub fn options(mut self, options: &'a Vec<String>) -> Self {
        self.options = options;
        self
    }
    pub fn get_options(&'a self) -> &'a &'a Vec<String> {
        &self.options
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "True, if the poll needs to be anonymous, defaults to True"]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }
    pub fn get_is_anonymous(&'a self) -> &'a Option<bool> {
        &self.is_anonymous
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Poll type, \"quiz\" or \"regular\", defaults to \"regular\""]
    pub fn tg_type(mut self, tg_type: &'a str) -> Self {
        self.tg_type = Some(tg_type);
        self
    }
    pub fn get_tg_type(&'a self) -> &'a Option<&'a str> {
        &self.tg_type
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False"]
    pub fn allows_multiple_answers(mut self, allows_multiple_answers: bool) -> Self {
        self.allows_multiple_answers = Some(allows_multiple_answers);
        self
    }
    pub fn get_allows_multiple_answers(&'a self) -> &'a Option<bool> {
        &self.allows_multiple_answers
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "0-based identifier of the correct answer option, required for polls in quiz mode"]
    pub fn correct_option_id(mut self, correct_option_id: i64) -> Self {
        self.correct_option_id = Some(correct_option_id);
        self
    }
    pub fn get_correct_option_id(&'a self) -> &'a Option<i64> {
        &self.correct_option_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing"]
    pub fn explanation(mut self, explanation: &'a str) -> Self {
        self.explanation = Some(explanation);
        self
    }
    pub fn get_explanation(&'a self) -> &'a Option<&'a str> {
        &self.explanation
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the explanation. See formatting options for more details."]
    pub fn explanation_parse_mode(mut self, explanation_parse_mode: &'a str) -> Self {
        self.explanation_parse_mode = Some(explanation_parse_mode);
        self
    }
    pub fn get_explanation_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.explanation_parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in the poll explanation, which can be specified instead of parse_mode"]
    pub fn explanation_entities(mut self, explanation_entities: &'a Vec<MessageEntity>) -> Self {
        self.explanation_entities = Some(explanation_entities);
        self
    }
    pub fn get_explanation_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.explanation_entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date."]
    pub fn open_period(mut self, open_period: i64) -> Self {
        self.open_period = Some(open_period);
        self
    }
    pub fn get_open_period(&'a self) -> &'a Option<i64> {
        &self.open_period
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period."]
    pub fn close_date(mut self, close_date: i64) -> Self {
        self.close_date = Some(close_date);
        self
    }
    pub fn get_close_date(&'a self) -> &'a Option<i64> {
        &self.close_date
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the poll needs to be immediately closed. This can be useful for poll preview."]
    pub fn is_closed(mut self, is_closed: bool) -> Self {
        self.is_closed = Some(is_closed);
        self
    }
    pub fn get_is_closed(&'a self) -> &'a Option<bool> {
        &self.is_closed
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_poll(
                self.chat_id,
                self.message_thread_id,
                &self.question,
                &self.options,
                self.is_anonymous,
                self.tg_type,
                self.allows_multiple_answers,
                self.correct_option_id,
                self.explanation,
                self.explanation_parse_mode,
                self.explanation_entities,
                self.open_period,
                self.close_date,
                self.is_closed,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallGetUserProfilePhotos<'a> {
    bot: &'a Bot,
    user_id: i64,
    offset: Option<i64>,
    limit: Option<i64>,
}
impl<'a> CallGetUserProfilePhotos<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier of the target user"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sequential number of the first photo to be returned. By default, all photos are returned."]
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn get_offset(&'a self) -> &'a Option<i64> {
        &self.offset
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100."]
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn get_limit(&'a self) -> &'a Option<i64> {
        &self.limit
    }
    pub async fn build(self) -> BotResult<UserProfilePhotos> {
        self.bot
            .get_user_profile_photos(self.user_id, self.offset, self.limit)
            .await
    }
}
pub struct CallSetMyShortDescription<'a> {
    bot: &'a Bot,
    short_description: Option<&'a str>,
    language_code: Option<&'a str>,
}
impl<'a> CallSetMyShortDescription<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language."]
    pub fn short_description(mut self, short_description: &'a str) -> Self {
        self.short_description = Some(short_description);
        self
    }
    pub fn get_short_description(&'a self) -> &'a Option<&'a str> {
        &self.short_description
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description."]
    pub fn language_code(mut self, language_code: &'a str) -> Self {
        self.language_code = Some(language_code);
        self
    }
    pub fn get_language_code(&'a self) -> &'a Option<&'a str> {
        &self.language_code
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .set_my_short_description(self.short_description, self.language_code)
            .await
    }
}
pub struct CallCreateNewStickerSet<'a> {
    bot: &'a Bot,
    user_id: i64,
    name: &'a str,
    title: &'a str,
    stickers: &'a Vec<InputSticker>,
    sticker_format: &'a str,
    sticker_type: Option<&'a str>,
    needs_repainting: Option<bool>,
}
impl<'a> CallCreateNewStickerSet<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "User identifier of created sticker set owner"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in \"_by_<bot_username>\". <bot_username> is case insensitive. 1-64 characters."]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn get_name(&'a self) -> &'a &'a str {
        &self.name
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sticker set title, 1-64 characters"]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
    pub fn get_title(&'a self) -> &'a &'a str {
        &self.title
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of 1-50 initial stickers to be added to the sticker set"]
    pub fn stickers(mut self, stickers: &'a Vec<InputSticker>) -> Self {
        self.stickers = stickers;
        self
    }
    pub fn get_stickers(&'a self) -> &'a &'a Vec<InputSticker> {
        &self.stickers
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Format of stickers in the set, must be one of \"static\", \"animated\", \"video\""]
    pub fn sticker_format(mut self, sticker_format: &'a str) -> Self {
        self.sticker_format = sticker_format;
        self
    }
    pub fn get_sticker_format(&'a self) -> &'a &'a str {
        &self.sticker_format
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Type of stickers in the set, pass \"regular\", \"mask\", or \"custom_emoji\". By default, a regular sticker set is created."]
    pub fn sticker_type(mut self, sticker_type: &'a str) -> Self {
        self.sticker_type = Some(sticker_type);
        self
    }
    pub fn get_sticker_type(&'a self) -> &'a Option<&'a str> {
        &self.sticker_type
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only"]
    pub fn needs_repainting(mut self, needs_repainting: bool) -> Self {
        self.needs_repainting = Some(needs_repainting);
        self
    }
    pub fn get_needs_repainting(&'a self) -> &'a Option<bool> {
        &self.needs_repainting
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .create_new_sticker_set(
                self.user_id,
                &self.name,
                &self.title,
                &self.stickers,
                &self.sticker_format,
                self.sticker_type,
                self.needs_repainting,
            )
            .await
    }
}
pub struct CallAnswerShippingQuery<'a> {
    bot: &'a Bot,
    shipping_query_id: &'a str,
    ok: bool,
    shipping_options: Option<&'a Vec<ShippingOption>>,
    error_message: Option<&'a str>,
}
impl<'a> CallAnswerShippingQuery<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the query to be answered"]
    pub fn shipping_query_id(mut self, shipping_query_id: &'a str) -> Self {
        self.shipping_query_id = shipping_query_id;
        self
    }
    pub fn get_shipping_query_id(&'a self) -> &'a &'a str {
        &self.shipping_query_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)"]
    pub fn ok(mut self, ok: bool) -> Self {
        self.ok = ok;
        self
    }
    pub fn get_ok(&'a self) -> &'a bool {
        &self.ok
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if ok is True. A JSON-serialized array of available shipping options."]
    pub fn shipping_options(mut self, shipping_options: &'a Vec<ShippingOption>) -> Self {
        self.shipping_options = Some(shipping_options);
        self
    }
    pub fn get_shipping_options(&'a self) -> &'a Option<&'a Vec<ShippingOption>> {
        &self.shipping_options
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. \"Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user."]
    pub fn error_message(mut self, error_message: &'a str) -> Self {
        self.error_message = Some(error_message);
        self
    }
    pub fn get_error_message(&'a self) -> &'a Option<&'a str> {
        &self.error_message
    }
    pub async fn build(self) -> BotResult<bool> {
        self.bot
            .answer_shipping_query(
                &self.shipping_query_id,
                self.ok,
                self.shipping_options,
                self.error_message,
            )
            .await
    }
}
pub struct CallSendMessage<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    text: &'a str,
    parse_mode: Option<&'a str>,
    entities: Option<&'a Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendMessage<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Text of the message to be sent, 1-4096 characters after entities parsing"]
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }
    pub fn get_text(&'a self) -> &'a &'a str {
        &self.text
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Mode for parsing entities in the message text. See formatting options for more details."]
    pub fn parse_mode(mut self, parse_mode: &'a str) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn get_parse_mode(&'a self) -> &'a Option<&'a str> {
        &self.parse_mode
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of special entities that appear in message text, which can be specified instead of parse_mode"]
    pub fn entities(mut self, entities: &'a Vec<MessageEntity>) -> Self {
        self.entities = Some(entities);
        self
    }
    pub fn get_entities(&'a self) -> &'a Option<&'a Vec<MessageEntity>> {
        &self.entities
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Disables link previews for links in this message"]
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }
    pub fn get_disable_web_page_preview(&'a self) -> &'a Option<bool> {
        &self.disable_web_page_preview
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_message(
                self.chat_id,
                self.message_thread_id,
                &self.text,
                self.parse_mode,
                self.entities,
                self.disable_web_page_preview,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallGetUpdates<'a> {
    bot: &'a Bot,
    offset: Option<i64>,
    limit: Option<i64>,
    timeout: Option<i64>,
    allowed_updates: Option<&'a Vec<String>>,
}
impl<'a> CallGetUpdates<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will be forgotten."]
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn get_offset(&'a self) -> &'a Option<i64> {
        &self.offset
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100."]
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn get_limit(&'a self) -> &'a Option<i64> {
        &self.limit
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only."]
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn get_timeout(&'a self) -> &'a Option<i64> {
        &self.timeout
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A JSON-serialized list of the update types you want your bot to receive. For example, specify [\"message\", \"edited_channel_post\", \"callback_query\"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time."]
    pub fn allowed_updates(mut self, allowed_updates: &'a Vec<String>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }
    pub fn get_allowed_updates(&'a self) -> &'a Option<&'a Vec<String>> {
        &self.allowed_updates
    }
    pub async fn build(self) -> BotResult<Vec<Update>> {
        self.bot
            .get_updates(self.offset, self.limit, self.timeout, self.allowed_updates)
            .await
    }
}
pub struct CallSendLocation<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_thread_id: Option<i64>,
    latitude: ::ordered_float::OrderedFloat<f64>,
    longitude: ::ordered_float::OrderedFloat<f64>,
    horizontal_accuracy: Option<::ordered_float::OrderedFloat<f64>>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<&'a EReplyMarkup>,
}
impl<'a> CallSendLocation<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target chat or username of the target channel (in the format @channelusername)"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn get_chat_id(&'a self) -> &'a i64 {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Unique identifier for the target message thread (topic) of the forum; for forum supergroups only"]
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    pub fn get_message_thread_id(&'a self) -> &'a Option<i64> {
        &self.message_thread_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Latitude of the location"]
    pub fn latitude(mut self, latitude: ::ordered_float::OrderedFloat<f64>) -> Self {
        self.latitude = latitude;
        self
    }
    pub fn get_latitude(&'a self) -> &'a ::ordered_float::OrderedFloat<f64> {
        &self.latitude
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Longitude of the location"]
    pub fn longitude(mut self, longitude: ::ordered_float::OrderedFloat<f64>) -> Self {
        self.longitude = longitude;
        self
    }
    pub fn get_longitude(&'a self) -> &'a ::ordered_float::OrderedFloat<f64> {
        &self.longitude
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "The radius of uncertainty for the location, measured in meters; 0-1500"]
    pub fn horizontal_accuracy(
        mut self,
        horizontal_accuracy: ::ordered_float::OrderedFloat<f64>,
    ) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }
    pub fn get_horizontal_accuracy(&'a self) -> &'a Option<::ordered_float::OrderedFloat<f64>> {
        &self.horizontal_accuracy
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Period in seconds for which the location will be updated (see Live Locations, should be between 60 and 86400."]
    pub fn live_period(mut self, live_period: i64) -> Self {
        self.live_period = Some(live_period);
        self
    }
    pub fn get_live_period(&'a self) -> &'a Option<i64> {
        &self.live_period
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified."]
    pub fn heading(mut self, heading: i64) -> Self {
        self.heading = Some(heading);
        self
    }
    pub fn get_heading(&'a self) -> &'a Option<i64> {
        &self.heading
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified."]
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: i64) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }
    pub fn get_proximity_alert_radius(&'a self) -> &'a Option<i64> {
        &self.proximity_alert_radius
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Sends the message silently. Users will receive a notification with no sound."]
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn get_disable_notification(&'a self) -> &'a Option<bool> {
        &self.disable_notification
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Protects the contents of the sent message from forwarding and saving"]
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
    pub fn get_protect_content(&'a self) -> &'a Option<bool> {
        &self.protect_content
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If the message is a reply, ID of the original message"]
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn get_reply_to_message_id(&'a self) -> &'a Option<i64> {
        &self.reply_to_message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the message should be sent even if the specified replied-to message is not found"]
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn get_allow_sending_without_reply(&'a self) -> &'a Option<bool> {
        &self.allow_sending_without_reply
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user."]
    pub fn reply_markup(mut self, reply_markup: &'a EReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
    pub fn get_reply_markup(&'a self) -> &'a Option<&'a EReplyMarkup> {
        &self.reply_markup
    }
    pub async fn build(self) -> BotResult<Message> {
        self.bot
            .send_location(
                self.chat_id,
                self.message_thread_id,
                self.latitude,
                self.longitude,
                self.horizontal_accuracy,
                self.live_period,
                self.heading,
                self.proximity_alert_radius,
                self.disable_notification,
                self.protect_content,
                self.reply_to_message_id,
                self.allow_sending_without_reply,
                self.reply_markup,
            )
            .await
    }
}
pub struct CallSetGameScore<'a> {
    bot: &'a Bot,
    user_id: i64,
    score: i64,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
}
impl<'a> CallSetGameScore<'a> {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "User identifier"]
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn get_user_id(&'a self) -> &'a i64 {
        &self.user_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "New score, must be non-negative"]
    pub fn score(mut self, score: i64) -> Self {
        self.score = score;
        self
    }
    pub fn get_score(&'a self) -> &'a i64 {
        &self.score
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    pub fn get_force(&'a self) -> &'a Option<bool> {
        &self.force
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Pass True if the game message should not be automatically edited to include the current scoreboard"]
    pub fn disable_edit_message(mut self, disable_edit_message: bool) -> Self {
        self.disable_edit_message = Some(disable_edit_message);
        self
    }
    pub fn get_disable_edit_message(&'a self) -> &'a Option<bool> {
        &self.disable_edit_message
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Unique identifier for the target chat"]
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn get_chat_id(&'a self) -> &'a Option<i64> {
        &self.chat_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if inline_message_id is not specified. Identifier of the sent message"]
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn get_message_id(&'a self) -> &'a Option<i64> {
        &self.message_id
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Required if chat_id and message_id are not specified. Identifier of the inline message"]
    pub fn inline_message_id(mut self, inline_message_id: &'a str) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn get_inline_message_id(&'a self) -> &'a Option<&'a str> {
        &self.inline_message_id
    }
    pub async fn build(self) -> BotResult<MessageBool> {
        self.bot
            .set_game_score(
                self.user_id,
                self.score,
                self.force,
                self.disable_edit_message,
                self.chat_id,
                self.message_id,
                self.inline_message_id,
            )
            .await
    }
}
impl ApproveChatJoinRequestOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        form
    }
}
impl DeleteForumTopicOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "message_thread_id",
            self.message_thread_id.to_string()
        );
        let form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}
impl SendVideoNoteOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "video_note", self.video_note.to_string());
        let form = form.text("video_note", self.video_note.to_string());
        let form = if let Some(duration) = self.duration {
            form.text("duration", duration.to_string())
        } else {
            form
        };
        let form = if let Some(length) = self.length {
            form.text("length", length.to_string())
        } else {
            form
        };
        let form = if let Some(thumbnail) = self.thumbnail {
            form.text("thumbnail", thumbnail.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl PinChatMessageOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "message_id", self.message_id.to_string());
        let form = form.text("message_id", self.message_id.to_string());
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SendAudioOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "audio", self.audio.to_string());
        let form = form.text("audio", self.audio.to_string());
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(duration) = self.duration {
            form.text("duration", duration.to_string())
        } else {
            form
        };
        let form = if let Some(performer) = self.performer {
            form.text("performer", performer.to_string())
        } else {
            form
        };
        let form = if let Some(title) = self.title {
            form.text("title", title.to_string())
        } else {
            form
        };
        let form = if let Some(thumbnail) = self.thumbnail {
            form.text("thumbnail", thumbnail.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> SetMyCommandsOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "commands", self.commands);
        let form = form.text("commands", self.commands);
        let form = if let Some(scope) = self.scope {
            form.text("scope", scope)
        } else {
            form
        };
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl GetMeOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        form
    }
}
impl GetCustomEmojiStickersOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "custom_emoji_ids", self.custom_emoji_ids);
        let form = form.text("custom_emoji_ids", self.custom_emoji_ids);
        form
    }
}
impl<'a> AnswerWebAppQueryOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "result", self.result);
        let form = form.text("result", self.result);
        println!(
            "{} => {}",
            "web_app_query_id",
            self.web_app_query_id.to_string()
        );
        let form = form.text("web_app_query_id", self.web_app_query_id.to_string());
        form
    }
}
impl RestrictChatMemberOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "permissions", self.permissions);
        let form = form.text("permissions", self.permissions);
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        let form =
            if let Some(use_independent_chat_permissions) = self.use_independent_chat_permissions {
                form.text(
                    "use_independent_chat_permissions",
                    use_independent_chat_permissions.to_string(),
                )
            } else {
                form
            };
        let form = if let Some(until_date) = self.until_date {
            form.text("until_date", until_date.to_string())
        } else {
            form
        };
        form
    }
}
impl UnpinChatMessageOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SetChatAdministratorCustomTitleOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        println!("{} => {}", "custom_title", self.custom_title.to_string());
        let form = form.text("custom_title", self.custom_title.to_string());
        form
    }
}
impl<'a> SendDocumentOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "document", self.document.to_string());
        let form = form.text("document", self.document.to_string());
        let form = if let Some(thumbnail) = self.thumbnail {
            form.text("thumbnail", thumbnail.to_string())
        } else {
            form
        };
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(disable_content_type_detection) = self.disable_content_type_detection
        {
            form.text(
                "disable_content_type_detection",
                disable_content_type_detection.to_string(),
            )
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> EditGeneralForumTopicOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        form
    }
}
impl<'a> SetCustomEmojiStickerSetThumbnailOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        let form = if let Some(custom_emoji_id) = self.custom_emoji_id {
            form.text("custom_emoji_id", custom_emoji_id.to_string())
        } else {
            form
        };
        form
    }
}
impl SendMediaGroupOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "media", self.media);
        let form = form.text("media", self.media);
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> SetMyDescriptionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(description) = self.description {
            form.text("description", description.to_string())
        } else {
            form
        };
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SetStickerSetThumbnailOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        let form = if let Some(thumbnail) = self.thumbnail {
            form.text("thumbnail", thumbnail.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> UploadStickerFileOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "sticker", self.sticker);
        let form = form.text("sticker", self.sticker);
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        println!(
            "{} => {}",
            "sticker_format",
            self.sticker_format.to_string()
        );
        let form = form.text("sticker_format", self.sticker_format.to_string());
        form
    }
}
impl<'a> EditMessageReplyMarkupOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        form
    }
}
impl GetWebhookInfoOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        form
    }
}
impl SetChatPermissionsOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "permissions", self.permissions);
        let form = form.text("permissions", self.permissions);
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form =
            if let Some(use_independent_chat_permissions) = self.use_independent_chat_permissions {
                form.text(
                    "use_independent_chat_permissions",
                    use_independent_chat_permissions.to_string(),
                )
            } else {
                form
            };
        form
    }
}
impl UnpinAllForumTopicMessagesOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "message_thread_id",
            self.message_thread_id.to_string()
        );
        let form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}
impl GetChatOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> SetStickerEmojiListOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "emoji_list", self.emoji_list);
        let form = form.text("emoji_list", self.emoji_list);
        println!("{} => {}", "sticker", self.sticker.to_string());
        let form = form.text("sticker", self.sticker.to_string());
        form
    }
}
impl ForwardMessageOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "from_chat_id", self.from_chat_id.to_string());
        let form = form.text("from_chat_id", self.from_chat_id.to_string());
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        println!("{} => {}", "message_id", self.message_id.to_string());
        let form = form.text("message_id", self.message_id.to_string());
        form
    }
}
impl<'a> SendContactOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "phone_number", self.phone_number.to_string());
        let form = form.text("phone_number", self.phone_number.to_string());
        println!("{} => {}", "first_name", self.first_name.to_string());
        let form = form.text("first_name", self.first_name.to_string());
        let form = if let Some(last_name) = self.last_name {
            form.text("last_name", last_name.to_string())
        } else {
            form
        };
        let form = if let Some(vcard) = self.vcard {
            form.text("vcard", vcard.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl DeleteChatPhotoOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl GetChatMemberOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        form
    }
}
impl HideGeneralForumTopicOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> EditMessageTextOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(entities) = self.entities {
            form.text("entities", entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "text", self.text.to_string());
        let form = form.text("text", self.text.to_string());
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(disable_web_page_preview) = self.disable_web_page_preview {
            form.text(
                "disable_web_page_preview",
                disable_web_page_preview.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> SetStickerPositionInSetOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "sticker", self.sticker.to_string());
        let form = form.text("sticker", self.sticker.to_string());
        println!("{} => {}", "position", self.position.to_string());
        let form = form.text("position", self.position.to_string());
        form
    }
}
impl<'a> SendVideoOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "video", self.video.to_string());
        let form = form.text("video", self.video.to_string());
        let form = if let Some(duration) = self.duration {
            form.text("duration", duration.to_string())
        } else {
            form
        };
        let form = if let Some(width) = self.width {
            form.text("width", width.to_string())
        } else {
            form
        };
        let form = if let Some(height) = self.height {
            form.text("height", height.to_string())
        } else {
            form
        };
        let form = if let Some(thumbnail) = self.thumbnail {
            form.text("thumbnail", thumbnail.to_string())
        } else {
            form
        };
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(has_spoiler) = self.has_spoiler {
            form.text("has_spoiler", has_spoiler.to_string())
        } else {
            form
        };
        let form = if let Some(supports_streaming) = self.supports_streaming {
            form.text("supports_streaming", supports_streaming.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> AnswerInlineQueryOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "results", self.results);
        let form = form.text("results", self.results);
        let form = if let Some(button) = self.button {
            form.text("button", button)
        } else {
            form
        };
        println!(
            "{} => {}",
            "inline_query_id",
            self.inline_query_id.to_string()
        );
        let form = form.text("inline_query_id", self.inline_query_id.to_string());
        let form = if let Some(cache_time) = self.cache_time {
            form.text("cache_time", cache_time.to_string())
        } else {
            form
        };
        let form = if let Some(is_personal) = self.is_personal {
            form.text("is_personal", is_personal.to_string())
        } else {
            form
        };
        let form = if let Some(next_offset) = self.next_offset {
            form.text("next_offset", next_offset.to_string())
        } else {
            form
        };
        form
    }
}
impl BanChatSenderChatOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "sender_chat_id",
            self.sender_chat_id.to_string()
        );
        let form = form.text("sender_chat_id", self.sender_chat_id.to_string());
        form
    }
}
impl PromoteChatMemberOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        let form = if let Some(is_anonymous) = self.is_anonymous {
            form.text("is_anonymous", is_anonymous.to_string())
        } else {
            form
        };
        let form = if let Some(can_manage_chat) = self.can_manage_chat {
            form.text("can_manage_chat", can_manage_chat.to_string())
        } else {
            form
        };
        let form = if let Some(can_delete_messages) = self.can_delete_messages {
            form.text("can_delete_messages", can_delete_messages.to_string())
        } else {
            form
        };
        let form = if let Some(can_manage_video_chats) = self.can_manage_video_chats {
            form.text("can_manage_video_chats", can_manage_video_chats.to_string())
        } else {
            form
        };
        let form = if let Some(can_restrict_members) = self.can_restrict_members {
            form.text("can_restrict_members", can_restrict_members.to_string())
        } else {
            form
        };
        let form = if let Some(can_promote_members) = self.can_promote_members {
            form.text("can_promote_members", can_promote_members.to_string())
        } else {
            form
        };
        let form = if let Some(can_change_info) = self.can_change_info {
            form.text("can_change_info", can_change_info.to_string())
        } else {
            form
        };
        let form = if let Some(can_invite_users) = self.can_invite_users {
            form.text("can_invite_users", can_invite_users.to_string())
        } else {
            form
        };
        let form = if let Some(can_post_messages) = self.can_post_messages {
            form.text("can_post_messages", can_post_messages.to_string())
        } else {
            form
        };
        let form = if let Some(can_edit_messages) = self.can_edit_messages {
            form.text("can_edit_messages", can_edit_messages.to_string())
        } else {
            form
        };
        let form = if let Some(can_pin_messages) = self.can_pin_messages {
            form.text("can_pin_messages", can_pin_messages.to_string())
        } else {
            form
        };
        let form = if let Some(can_post_stories) = self.can_post_stories {
            form.text("can_post_stories", can_post_stories.to_string())
        } else {
            form
        };
        let form = if let Some(can_edit_stories) = self.can_edit_stories {
            form.text("can_edit_stories", can_edit_stories.to_string())
        } else {
            form
        };
        let form = if let Some(can_delete_stories) = self.can_delete_stories {
            form.text("can_delete_stories", can_delete_stories.to_string())
        } else {
            form
        };
        let form = if let Some(can_manage_topics) = self.can_manage_topics {
            form.text("can_manage_topics", can_manage_topics.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> DeleteMyCommandsOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(scope) = self.scope {
            form.text("scope", scope)
        } else {
            form
        };
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl ExportChatInviteLinkOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl GetChatMenuButtonOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> EditMessageLiveLocationOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "latitude", self.latitude.to_string());
        let form = form.text("latitude", self.latitude.to_string());
        println!("{} => {}", "longitude", self.longitude.to_string());
        let form = form.text("longitude", self.longitude.to_string());
        let form = if let Some(horizontal_accuracy) = self.horizontal_accuracy {
            form.text("horizontal_accuracy", horizontal_accuracy.to_string())
        } else {
            form
        };
        let form = if let Some(heading) = self.heading {
            form.text("heading", heading.to_string())
        } else {
            form
        };
        let form = if let Some(proximity_alert_radius) = self.proximity_alert_radius {
            form.text("proximity_alert_radius", proximity_alert_radius.to_string())
        } else {
            form
        };
        form
    }
}
impl DeleteMessageOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "message_id", self.message_id.to_string());
        let form = form.text("message_id", self.message_id.to_string());
        form
    }
}
impl<'a> SetStickerMaskPositionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(mask_position) = self.mask_position {
            form.text("mask_position", mask_position)
        } else {
            form
        };
        println!("{} => {}", "sticker", self.sticker.to_string());
        let form = form.text("sticker", self.sticker.to_string());
        form
    }
}
impl<'a> SetChatTitleOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "title", self.title.to_string());
        let form = form.text("title", self.title.to_string());
        form
    }
}
impl SetPassportDataErrorsOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "errors", self.errors);
        let form = form.text("errors", self.errors);
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        form
    }
}
impl CloseOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        form
    }
}
impl<'a> SendStickerOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "sticker", self.sticker.to_string());
        let form = form.text("sticker", self.sticker.to_string());
        let form = if let Some(emoji) = self.emoji {
            form.text("emoji", emoji.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl DeleteWebhookOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(drop_pending_updates) = self.drop_pending_updates {
            form.text("drop_pending_updates", drop_pending_updates.to_string())
        } else {
            form
        };
        form
    }
}
impl SetMyDefaultAdministratorRightsOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(rights) = self.rights {
            form.text("rights", rights)
        } else {
            form
        };
        let form = if let Some(for_channels) = self.for_channels {
            form.text("for_channels", for_channels.to_string())
        } else {
            form
        };
        form
    }
}
impl UnpinAllGeneralForumTopicMessagesOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> EditChatInviteLinkOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "invite_link", self.invite_link.to_string());
        let form = form.text("invite_link", self.invite_link.to_string());
        let form = if let Some(name) = self.name {
            form.text("name", name.to_string())
        } else {
            form
        };
        let form = if let Some(expire_date) = self.expire_date {
            form.text("expire_date", expire_date.to_string())
        } else {
            form
        };
        let form = if let Some(member_limit) = self.member_limit {
            form.text("member_limit", member_limit.to_string())
        } else {
            form
        };
        let form = if let Some(creates_join_request) = self.creates_join_request {
            form.text("creates_join_request", creates_join_request.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> EditMessageCaptionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> DeleteStickerFromSetOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "sticker", self.sticker.to_string());
        let form = form.text("sticker", self.sticker.to_string());
        form
    }
}
impl<'a> SendAnimationOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "animation", self.animation.to_string());
        let form = form.text("animation", self.animation.to_string());
        let form = if let Some(duration) = self.duration {
            form.text("duration", duration.to_string())
        } else {
            form
        };
        let form = if let Some(width) = self.width {
            form.text("width", width.to_string())
        } else {
            form
        };
        let form = if let Some(height) = self.height {
            form.text("height", height.to_string())
        } else {
            form
        };
        let form = if let Some(thumbnail) = self.thumbnail {
            form.text("thumbnail", thumbnail.to_string())
        } else {
            form
        };
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(has_spoiler) = self.has_spoiler {
            form.text("has_spoiler", has_spoiler.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> SetChatStickerSetOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "sticker_set_name",
            self.sticker_set_name.to_string()
        );
        let form = form.text("sticker_set_name", self.sticker_set_name.to_string());
        form
    }
}
impl<'a> SendChatActionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "action", self.action.to_string());
        let form = form.text("action", self.action.to_string());
        form
    }
}
impl<'a> GetFileOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "file_id", self.file_id.to_string());
        let form = form.text("file_id", self.file_id.to_string());
        form
    }
}
impl<'a> AddStickerToSetOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "sticker", self.sticker);
        let form = form.text("sticker", self.sticker);
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        form
    }
}
impl<'a> GetMyNameOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SetStickerSetTitleOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        println!("{} => {}", "title", self.title.to_string());
        let form = form.text("title", self.title.to_string());
        form
    }
}
impl CloseForumTopicOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "message_thread_id",
            self.message_thread_id.to_string()
        );
        let form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}
impl<'a> GetMyCommandsOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(scope) = self.scope {
            form.text("scope", scope)
        } else {
            form
        };
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> CopyMessageOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "from_chat_id", self.from_chat_id.to_string());
        let form = form.text("from_chat_id", self.from_chat_id.to_string());
        println!("{} => {}", "message_id", self.message_id.to_string());
        let form = form.text("message_id", self.message_id.to_string());
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl UnpinAllChatMessagesOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl GetChatMemberCountOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> AnswerPreCheckoutQueryOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!(
            "{} => {}",
            "pre_checkout_query_id",
            self.pre_checkout_query_id.to_string()
        );
        let form = form.text(
            "pre_checkout_query_id",
            self.pre_checkout_query_id.to_string(),
        );
        println!("{} => {}", "ok", self.ok.to_string());
        let form = form.text("ok", self.ok.to_string());
        let form = if let Some(error_message) = self.error_message {
            form.text("error_message", error_message.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> CreateChatInviteLinkOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(name) = self.name {
            form.text("name", name.to_string())
        } else {
            form
        };
        let form = if let Some(expire_date) = self.expire_date {
            form.text("expire_date", expire_date.to_string())
        } else {
            form
        };
        let form = if let Some(member_limit) = self.member_limit {
            form.text("member_limit", member_limit.to_string())
        } else {
            form
        };
        let form = if let Some(creates_join_request) = self.creates_join_request {
            form.text("creates_join_request", creates_join_request.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> EditForumTopicOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "message_thread_id",
            self.message_thread_id.to_string()
        );
        let form = form.text("message_thread_id", self.message_thread_id.to_string());
        let form = if let Some(name) = self.name {
            form.text("name", name.to_string())
        } else {
            form
        };
        let form = if let Some(icon_custom_emoji_id) = self.icon_custom_emoji_id {
            form.text("icon_custom_emoji_id", icon_custom_emoji_id.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SetWebhookOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(certificate) = self.certificate {
            form.text("certificate", certificate)
        } else {
            form
        };
        let form = if let Some(allowed_updates) = self.allowed_updates {
            form.text("allowed_updates", allowed_updates)
        } else {
            form
        };
        println!("{} => {}", "url", self.url.to_string());
        let form = form.text("url", self.url.to_string());
        let form = if let Some(ip_address) = self.ip_address {
            form.text("ip_address", ip_address.to_string())
        } else {
            form
        };
        let form = if let Some(max_connections) = self.max_connections {
            form.text("max_connections", max_connections.to_string())
        } else {
            form
        };
        let form = if let Some(drop_pending_updates) = self.drop_pending_updates {
            form.text("drop_pending_updates", drop_pending_updates.to_string())
        } else {
            form
        };
        let form = if let Some(secret_token) = self.secret_token {
            form.text("secret_token", secret_token.to_string())
        } else {
            form
        };
        form
    }
}
impl LogOutOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        form
    }
}
impl<'a> RevokeChatInviteLinkOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "invite_link", self.invite_link.to_string());
        let form = form.text("invite_link", self.invite_link.to_string());
        form
    }
}
impl ReopenForumTopicOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "message_thread_id",
            self.message_thread_id.to_string()
        );
        let form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}
impl<'a> GetMyShortDescriptionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl GetMyDefaultAdministratorRightsOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(for_channels) = self.for_channels {
            form.text("for_channels", for_channels.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SetStickerKeywordsOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(keywords) = self.keywords {
            form.text("keywords", keywords)
        } else {
            form
        };
        println!("{} => {}", "sticker", self.sticker.to_string());
        let form = form.text("sticker", self.sticker.to_string());
        form
    }
}
impl SetChatMenuButtonOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(menu_button) = self.menu_button {
            form.text("menu_button", menu_button)
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> EditMessageMediaOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "media", self.media);
        let form = form.text("media", self.media);
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        form
    }
}
impl ReopenGeneralForumTopicOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> GetGameHighScoresOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> CreateForumTopicOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        let form = if let Some(icon_color) = self.icon_color {
            form.text("icon_color", icon_color.to_string())
        } else {
            form
        };
        let form = if let Some(icon_custom_emoji_id) = self.icon_custom_emoji_id {
            form.text("icon_custom_emoji_id", icon_custom_emoji_id.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SendInvoiceOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "prices", self.prices);
        let form = form.text("prices", self.prices);
        let form = if let Some(suggested_tip_amounts) = self.suggested_tip_amounts {
            form.text("suggested_tip_amounts", suggested_tip_amounts)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "title", self.title.to_string());
        let form = form.text("title", self.title.to_string());
        println!("{} => {}", "description", self.description.to_string());
        let form = form.text("description", self.description.to_string());
        println!("{} => {}", "payload", self.payload.to_string());
        let form = form.text("payload", self.payload.to_string());
        println!(
            "{} => {}",
            "provider_token",
            self.provider_token.to_string()
        );
        let form = form.text("provider_token", self.provider_token.to_string());
        println!("{} => {}", "currency", self.currency.to_string());
        let form = form.text("currency", self.currency.to_string());
        let form = if let Some(max_tip_amount) = self.max_tip_amount {
            form.text("max_tip_amount", max_tip_amount.to_string())
        } else {
            form
        };
        let form = if let Some(start_parameter) = self.start_parameter {
            form.text("start_parameter", start_parameter.to_string())
        } else {
            form
        };
        let form = if let Some(provider_data) = self.provider_data {
            form.text("provider_data", provider_data.to_string())
        } else {
            form
        };
        let form = if let Some(photo_url) = self.photo_url {
            form.text("photo_url", photo_url.to_string())
        } else {
            form
        };
        let form = if let Some(photo_size) = self.photo_size {
            form.text("photo_size", photo_size.to_string())
        } else {
            form
        };
        let form = if let Some(photo_width) = self.photo_width {
            form.text("photo_width", photo_width.to_string())
        } else {
            form
        };
        let form = if let Some(photo_height) = self.photo_height {
            form.text("photo_height", photo_height.to_string())
        } else {
            form
        };
        let form = if let Some(need_name) = self.need_name {
            form.text("need_name", need_name.to_string())
        } else {
            form
        };
        let form = if let Some(need_phone_number) = self.need_phone_number {
            form.text("need_phone_number", need_phone_number.to_string())
        } else {
            form
        };
        let form = if let Some(need_email) = self.need_email {
            form.text("need_email", need_email.to_string())
        } else {
            form
        };
        let form = if let Some(need_shipping_address) = self.need_shipping_address {
            form.text("need_shipping_address", need_shipping_address.to_string())
        } else {
            form
        };
        let form = if let Some(send_phone_number_to_provider) = self.send_phone_number_to_provider {
            form.text(
                "send_phone_number_to_provider",
                send_phone_number_to_provider.to_string(),
            )
        } else {
            form
        };
        let form = if let Some(send_email_to_provider) = self.send_email_to_provider {
            form.text("send_email_to_provider", send_email_to_provider.to_string())
        } else {
            form
        };
        let form = if let Some(is_flexible) = self.is_flexible {
            form.text("is_flexible", is_flexible.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl LeaveChatOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl DeleteChatStickerSetOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> GetStickerSetOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        form
    }
}
impl<'a> GetMyDescriptionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SendPhotoOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "photo", self.photo.to_string());
        let form = form.text("photo", self.photo.to_string());
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(has_spoiler) = self.has_spoiler {
            form.text("has_spoiler", has_spoiler.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl CloseGeneralForumTopicOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> SetChatDescriptionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(description) = self.description {
            form.text("description", description.to_string())
        } else {
            form
        };
        form
    }
}
impl GetChatAdministratorsOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl GetForumTopicIconStickersOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        form
    }
}
impl UnhideGeneralForumTopicOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl StopPollOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "message_id", self.message_id.to_string());
        let form = form.text("message_id", self.message_id.to_string());
        form
    }
}
impl BanChatMemberOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        let form = if let Some(until_date) = self.until_date {
            form.text("until_date", until_date.to_string())
        } else {
            form
        };
        let form = if let Some(revoke_messages) = self.revoke_messages {
            form.text("revoke_messages", revoke_messages.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SendVenueOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "latitude", self.latitude.to_string());
        let form = form.text("latitude", self.latitude.to_string());
        println!("{} => {}", "longitude", self.longitude.to_string());
        let form = form.text("longitude", self.longitude.to_string());
        println!("{} => {}", "title", self.title.to_string());
        let form = form.text("title", self.title.to_string());
        println!("{} => {}", "address", self.address.to_string());
        let form = form.text("address", self.address.to_string());
        let form = if let Some(foursquare_id) = self.foursquare_id {
            form.text("foursquare_id", foursquare_id.to_string())
        } else {
            form
        };
        let form = if let Some(foursquare_type) = self.foursquare_type {
            form.text("foursquare_type", foursquare_type.to_string())
        } else {
            form
        };
        let form = if let Some(google_place_id) = self.google_place_id {
            form.text("google_place_id", google_place_id.to_string())
        } else {
            form
        };
        let form = if let Some(google_place_type) = self.google_place_type {
            form.text("google_place_type", google_place_type.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl DeclineChatJoinRequestOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        form
    }
}
impl SetChatPhotoOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "photo", self.photo);
        let form = form.text("photo", self.photo);
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}
impl<'a> CreateInvoiceLinkOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "prices", self.prices);
        let form = form.text("prices", self.prices);
        let form = if let Some(suggested_tip_amounts) = self.suggested_tip_amounts {
            form.text("suggested_tip_amounts", suggested_tip_amounts)
        } else {
            form
        };
        println!("{} => {}", "title", self.title.to_string());
        let form = form.text("title", self.title.to_string());
        println!("{} => {}", "description", self.description.to_string());
        let form = form.text("description", self.description.to_string());
        println!("{} => {}", "payload", self.payload.to_string());
        let form = form.text("payload", self.payload.to_string());
        println!(
            "{} => {}",
            "provider_token",
            self.provider_token.to_string()
        );
        let form = form.text("provider_token", self.provider_token.to_string());
        println!("{} => {}", "currency", self.currency.to_string());
        let form = form.text("currency", self.currency.to_string());
        let form = if let Some(max_tip_amount) = self.max_tip_amount {
            form.text("max_tip_amount", max_tip_amount.to_string())
        } else {
            form
        };
        let form = if let Some(provider_data) = self.provider_data {
            form.text("provider_data", provider_data.to_string())
        } else {
            form
        };
        let form = if let Some(photo_url) = self.photo_url {
            form.text("photo_url", photo_url.to_string())
        } else {
            form
        };
        let form = if let Some(photo_size) = self.photo_size {
            form.text("photo_size", photo_size.to_string())
        } else {
            form
        };
        let form = if let Some(photo_width) = self.photo_width {
            form.text("photo_width", photo_width.to_string())
        } else {
            form
        };
        let form = if let Some(photo_height) = self.photo_height {
            form.text("photo_height", photo_height.to_string())
        } else {
            form
        };
        let form = if let Some(need_name) = self.need_name {
            form.text("need_name", need_name.to_string())
        } else {
            form
        };
        let form = if let Some(need_phone_number) = self.need_phone_number {
            form.text("need_phone_number", need_phone_number.to_string())
        } else {
            form
        };
        let form = if let Some(need_email) = self.need_email {
            form.text("need_email", need_email.to_string())
        } else {
            form
        };
        let form = if let Some(need_shipping_address) = self.need_shipping_address {
            form.text("need_shipping_address", need_shipping_address.to_string())
        } else {
            form
        };
        let form = if let Some(send_phone_number_to_provider) = self.send_phone_number_to_provider {
            form.text(
                "send_phone_number_to_provider",
                send_phone_number_to_provider.to_string(),
            )
        } else {
            form
        };
        let form = if let Some(send_email_to_provider) = self.send_email_to_provider {
            form.text("send_email_to_provider", send_email_to_provider.to_string())
        } else {
            form
        };
        let form = if let Some(is_flexible) = self.is_flexible {
            form.text("is_flexible", is_flexible.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> StopMessageLiveLocationOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SendGameOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!(
            "{} => {}",
            "game_short_name",
            self.game_short_name.to_string()
        );
        let form = form.text("game_short_name", self.game_short_name.to_string());
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl UnbanChatMemberOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        let form = if let Some(only_if_banned) = self.only_if_banned {
            form.text("only_if_banned", only_if_banned.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SendVoiceOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(caption_entities) = self.caption_entities {
            form.text("caption_entities", caption_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "voice", self.voice.to_string());
        let form = form.text("voice", self.voice.to_string());
        let form = if let Some(caption) = self.caption {
            form.text("caption", caption.to_string())
        } else {
            form
        };
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(duration) = self.duration {
            form.text("duration", duration.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> SendDiceOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        let form = if let Some(emoji) = self.emoji {
            form.text("emoji", emoji.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> AnswerCallbackQueryOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!(
            "{} => {}",
            "callback_query_id",
            self.callback_query_id.to_string()
        );
        let form = form.text("callback_query_id", self.callback_query_id.to_string());
        let form = if let Some(text) = self.text {
            form.text("text", text.to_string())
        } else {
            form
        };
        let form = if let Some(show_alert) = self.show_alert {
            form.text("show_alert", show_alert.to_string())
        } else {
            form
        };
        let form = if let Some(url) = self.url {
            form.text("url", url.to_string())
        } else {
            form
        };
        let form = if let Some(cache_time) = self.cache_time {
            form.text("cache_time", cache_time.to_string())
        } else {
            form
        };
        form
    }
}
impl UnbanChatSenderChatOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        println!(
            "{} => {}",
            "sender_chat_id",
            self.sender_chat_id.to_string()
        );
        let form = form.text("sender_chat_id", self.sender_chat_id.to_string());
        form
    }
}
impl<'a> SetMyNameOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(name) = self.name {
            form.text("name", name.to_string())
        } else {
            form
        };
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> DeleteStickerSetOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        form
    }
}
impl<'a> SendPollOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "options", self.options);
        let form = form.text("options", self.options);
        let form = if let Some(explanation_entities) = self.explanation_entities {
            form.text("explanation_entities", explanation_entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "question", self.question.to_string());
        let form = form.text("question", self.question.to_string());
        let form = if let Some(is_anonymous) = self.is_anonymous {
            form.text("is_anonymous", is_anonymous.to_string())
        } else {
            form
        };
        let form = if let Some(tg_type) = self.tg_type {
            form.text("tg_type", tg_type.to_string())
        } else {
            form
        };
        let form = if let Some(allows_multiple_answers) = self.allows_multiple_answers {
            form.text(
                "allows_multiple_answers",
                allows_multiple_answers.to_string(),
            )
        } else {
            form
        };
        let form = if let Some(correct_option_id) = self.correct_option_id {
            form.text("correct_option_id", correct_option_id.to_string())
        } else {
            form
        };
        let form = if let Some(explanation) = self.explanation {
            form.text("explanation", explanation.to_string())
        } else {
            form
        };
        let form = if let Some(explanation_parse_mode) = self.explanation_parse_mode {
            form.text("explanation_parse_mode", explanation_parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(open_period) = self.open_period {
            form.text("open_period", open_period.to_string())
        } else {
            form
        };
        let form = if let Some(close_date) = self.close_date {
            form.text("close_date", close_date.to_string())
        } else {
            form
        };
        let form = if let Some(is_closed) = self.is_closed {
            form.text("is_closed", is_closed.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl GetUserProfilePhotosOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        let form = if let Some(offset) = self.offset {
            form.text("offset", offset.to_string())
        } else {
            form
        };
        let form = if let Some(limit) = self.limit {
            form.text("limit", limit.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SetMyShortDescriptionOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(short_description) = self.short_description {
            form.text("short_description", short_description.to_string())
        } else {
            form
        };
        let form = if let Some(language_code) = self.language_code {
            form.text("language_code", language_code.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> CreateNewStickerSetOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "stickers", self.stickers);
        let form = form.text("stickers", self.stickers);
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        println!("{} => {}", "name", self.name.to_string());
        let form = form.text("name", self.name.to_string());
        println!("{} => {}", "title", self.title.to_string());
        let form = form.text("title", self.title.to_string());
        println!(
            "{} => {}",
            "sticker_format",
            self.sticker_format.to_string()
        );
        let form = form.text("sticker_format", self.sticker_format.to_string());
        let form = if let Some(sticker_type) = self.sticker_type {
            form.text("sticker_type", sticker_type.to_string())
        } else {
            form
        };
        let form = if let Some(needs_repainting) = self.needs_repainting {
            form.text("needs_repainting", needs_repainting.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> AnswerShippingQueryOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(shipping_options) = self.shipping_options {
            form.text("shipping_options", shipping_options)
        } else {
            form
        };
        println!(
            "{} => {}",
            "shipping_query_id",
            self.shipping_query_id.to_string()
        );
        let form = form.text("shipping_query_id", self.shipping_query_id.to_string());
        println!("{} => {}", "ok", self.ok.to_string());
        let form = form.text("ok", self.ok.to_string());
        let form = if let Some(error_message) = self.error_message {
            form.text("error_message", error_message.to_string())
        } else {
            form
        };
        form
    }
}
impl<'a> SendMessageOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(entities) = self.entities {
            form.text("entities", entities)
        } else {
            form
        };
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "text", self.text.to_string());
        let form = form.text("text", self.text.to_string());
        let form = if let Some(parse_mode) = self.parse_mode {
            form.text("parse_mode", parse_mode.to_string())
        } else {
            form
        };
        let form = if let Some(disable_web_page_preview) = self.disable_web_page_preview {
            form.text(
                "disable_web_page_preview",
                disable_web_page_preview.to_string(),
            )
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl GetUpdatesOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(allowed_updates) = self.allowed_updates {
            form.text("allowed_updates", allowed_updates)
        } else {
            form
        };
        let form = if let Some(offset) = self.offset {
            form.text("offset", offset.to_string())
        } else {
            form
        };
        let form = if let Some(limit) = self.limit {
            form.text("limit", limit.to_string())
        } else {
            form
        };
        let form = if let Some(timeout) = self.timeout {
            form.text("timeout", timeout.to_string())
        } else {
            form
        };
        form
    }
}
impl SendLocationOpts {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        let form = if let Some(reply_markup) = self.reply_markup {
            form.text("reply_markup", reply_markup)
        } else {
            form
        };
        println!("{} => {}", "chat_id", self.chat_id.to_string());
        let form = form.text("chat_id", self.chat_id.to_string());
        let form = if let Some(message_thread_id) = self.message_thread_id {
            form.text("message_thread_id", message_thread_id.to_string())
        } else {
            form
        };
        println!("{} => {}", "latitude", self.latitude.to_string());
        let form = form.text("latitude", self.latitude.to_string());
        println!("{} => {}", "longitude", self.longitude.to_string());
        let form = form.text("longitude", self.longitude.to_string());
        let form = if let Some(horizontal_accuracy) = self.horizontal_accuracy {
            form.text("horizontal_accuracy", horizontal_accuracy.to_string())
        } else {
            form
        };
        let form = if let Some(live_period) = self.live_period {
            form.text("live_period", live_period.to_string())
        } else {
            form
        };
        let form = if let Some(heading) = self.heading {
            form.text("heading", heading.to_string())
        } else {
            form
        };
        let form = if let Some(proximity_alert_radius) = self.proximity_alert_radius {
            form.text("proximity_alert_radius", proximity_alert_radius.to_string())
        } else {
            form
        };
        let form = if let Some(disable_notification) = self.disable_notification {
            form.text("disable_notification", disable_notification.to_string())
        } else {
            form
        };
        let form = if let Some(protect_content) = self.protect_content {
            form.text("protect_content", protect_content.to_string())
        } else {
            form
        };
        let form = if let Some(reply_to_message_id) = self.reply_to_message_id {
            form.text("reply_to_message_id", reply_to_message_id.to_string())
        } else {
            form
        };
        let form = if let Some(allow_sending_without_reply) = self.allow_sending_without_reply {
            form.text(
                "allow_sending_without_reply",
                allow_sending_without_reply.to_string(),
            )
        } else {
            form
        };
        form
    }
}
impl<'a> SetGameScoreOpts<'a> {
    #[allow(dead_code)]
    fn get_form(self, form: Form) -> Form {
        println!("{} => {}", "user_id", self.user_id.to_string());
        let form = form.text("user_id", self.user_id.to_string());
        println!("{} => {}", "score", self.score.to_string());
        let form = form.text("score", self.score.to_string());
        let form = if let Some(force) = self.force {
            form.text("force", force.to_string())
        } else {
            form
        };
        let form = if let Some(disable_edit_message) = self.disable_edit_message {
            form.text("disable_edit_message", disable_edit_message.to_string())
        } else {
            form
        };
        let form = if let Some(chat_id) = self.chat_id {
            form.text("chat_id", chat_id.to_string())
        } else {
            form
        };
        let form = if let Some(message_id) = self.message_id {
            form.text("message_id", message_id.to_string())
        } else {
            form
        };
        let form = if let Some(inline_message_id) = self.inline_message_id {
            form.text("inline_message_id", inline_message_id.to_string())
        } else {
            form
        };
        form
    }
}
impl Bot {
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success."]
    pub async fn approve_chat_join_request<'a>(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> BotResult<bool> {
        let form = ApproveChatJoinRequestOpts {
            chat_id: chat_id,
            user_id: user_id,
        };
        let resp = self.post("approveChatJoinRequest", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_delete_messages administrator rights. Returns True on success."]
    pub async fn delete_forum_topic<'a>(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> BotResult<bool> {
        let form = DeleteForumTopicOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
        };
        let resp = self.post("deleteForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned."]
    pub async fn send_video_note<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        video_note: FileData,
        duration: Option<i64>,
        length: Option<i64>,
        thumbnail: Option<FileData>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, video_note_json) = video_note.to_form(data, "video_note".to_owned())?;
        let (data, thumbnail_json) = if let Some(thumbnail) = thumbnail {
            let (data, thumbnail_json) = thumbnail.to_form(data, "thumbnail".to_owned())?;
            (data, Some(thumbnail_json))
        } else {
            (data, None)
        };
        let form = SendVideoNoteOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            video_note: video_note_json,
            duration: duration,
            length: length,
            thumbnail: thumbnail_json,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendVideoNote", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success."]
    pub async fn pin_chat_message<'a>(
        &self,
        chat_id: i64,
        message_id: i64,
        disable_notification: Option<bool>,
    ) -> BotResult<bool> {
        let form = PinChatMessageOpts {
            chat_id: chat_id,
            message_id: message_id,
            disable_notification: disable_notification,
        };
        let resp = self.post("pinChatMessage", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.For sending voice messages, use the sendVoice method instead."]
    pub async fn send_audio<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        audio: FileData,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        duration: Option<i64>,
        performer: Option<&'a str>,
        title: Option<&'a str>,
        thumbnail: Option<FileData>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, audio_json) = audio.to_form(data, "audio".to_owned())?;
        let (data, thumbnail_json) = if let Some(thumbnail) = thumbnail {
            let (data, thumbnail_json) = thumbnail.to_form(data, "thumbnail".to_owned())?;
            (data, Some(thumbnail_json))
        } else {
            (data, None)
        };
        let form = SendAudioOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            audio: audio_json,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            duration: duration,
            performer: performer,
            title: title,
            thumbnail: thumbnail_json,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendAudio", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns True on success."]
    pub async fn set_my_commands<'a>(
        &self,
        commands: &'a Vec<BotCommand>,
        scope: Option<&'a BotCommandScope>,
        language_code: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = SetMyCommandsOpts {
            commands: serde_json::to_string(&commands)?,
            scope: if let Some(scope) = scope {
                Some(serde_json::to_string(&scope)?)
            } else {
                None
            },
            language_code: language_code,
        };
        let resp = self.post("setMyCommands", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object."]
    pub async fn get_me<'a>(&self) -> BotResult<User> {
        let resp = self.post_empty("getMe").await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects."]
    pub async fn get_custom_emoji_stickers<'a>(
        &self,
        custom_emoji_ids: &'a Vec<String>,
    ) -> BotResult<Vec<Sticker>> {
        let form = GetCustomEmojiStickersOpts {
            custom_emoji_ids: serde_json::to_string(&custom_emoji_ids)?,
        };
        let resp = self.post("getCustomEmojiStickers", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned."]
    pub async fn answer_web_app_query<'a>(
        &self,
        web_app_query_id: &'a str,
        result: &'a InlineQueryResult,
    ) -> BotResult<SentWebAppMessage> {
        let form = AnswerWebAppQueryOpts {
            web_app_query_id: web_app_query_id,
            result: serde_json::to_string(&result)?,
        };
        let resp = self.post("answerWebAppQuery", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success."]
    pub async fn restrict_chat_member<'a>(
        &self,
        chat_id: i64,
        user_id: i64,
        permissions: &'a ChatPermissions,
        use_independent_chat_permissions: Option<bool>,
        until_date: Option<i64>,
    ) -> BotResult<bool> {
        let form = RestrictChatMemberOpts {
            chat_id: chat_id,
            user_id: user_id,
            permissions: serde_json::to_string(&permissions)?,
            use_independent_chat_permissions: use_independent_chat_permissions,
            until_date: until_date,
        };
        let resp = self.post("restrictChatMember", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success."]
    pub async fn unpin_chat_message<'a>(
        &self,
        chat_id: i64,
        message_id: Option<i64>,
    ) -> BotResult<bool> {
        let form = UnpinChatMessageOpts {
            chat_id: chat_id,
            message_id: message_id,
        };
        let resp = self.post("unpinChatMessage", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success."]
    pub async fn set_chat_administrator_custom_title<'a>(
        &self,
        chat_id: i64,
        user_id: i64,
        custom_title: &'a str,
    ) -> BotResult<bool> {
        let form = SetChatAdministratorCustomTitleOpts {
            chat_id: chat_id,
            user_id: user_id,
            custom_title: custom_title,
        };
        let resp = self.post("setChatAdministratorCustomTitle", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future."]
    pub async fn send_document<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        document: FileData,
        thumbnail: Option<FileData>,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        disable_content_type_detection: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, document_json) = document.to_form(data, "document".to_owned())?;
        let (data, thumbnail_json) = if let Some(thumbnail) = thumbnail {
            let (data, thumbnail_json) = thumbnail.to_form(data, "thumbnail".to_owned())?;
            (data, Some(thumbnail_json))
        } else {
            (data, None)
        };
        let form = SendDocumentOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            document: document_json,
            thumbnail: thumbnail_json,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            disable_content_type_detection: disable_content_type_detection,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendDocument", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights. Returns True on success."]
    pub async fn edit_general_forum_topic<'a>(
        &self,
        chat_id: i64,
        name: &'a str,
    ) -> BotResult<bool> {
        let form = EditGeneralForumTopicOpts {
            chat_id: chat_id,
            name: name,
        };
        let resp = self.post("editGeneralForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success."]
    pub async fn set_custom_emoji_sticker_set_thumbnail<'a>(
        &self,
        name: &'a str,
        custom_emoji_id: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = SetCustomEmojiStickerSetThumbnailOpts {
            name: name,
            custom_emoji_id: custom_emoji_id,
        };
        let resp = self.post("setCustomEmojiStickerSetThumbnail", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned."]
    pub async fn send_media_group<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        media: &'a Vec<EMedia>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
    ) -> BotResult<Vec<Message>> {
        let form = SendMediaGroupOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            media: serde_json::to_string(&media)?,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
        };
        let resp = self.post("sendMediaGroup", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns True on success."]
    pub async fn set_my_description<'a>(
        &self,
        description: Option<&'a str>,
        language_code: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = SetMyDescriptionOpts {
            description: description,
            language_code: language_code,
        };
        let resp = self.post("setMyDescription", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns True on success."]
    pub async fn set_sticker_set_thumbnail<'a>(
        &self,
        name: &'a str,
        user_id: i64,
        thumbnail: Option<FileData>,
    ) -> BotResult<bool> {
        let data = Form::new();
        let (data, thumbnail_json) = if let Some(thumbnail) = thumbnail {
            let (data, thumbnail_json) = thumbnail.to_form(data, "thumbnail".to_owned())?;
            (data, Some(thumbnail_json))
        } else {
            (data, None)
        };
        let form = SetStickerSetThumbnailOpts {
            name: name,
            user_id: user_id,
            thumbnail: thumbnail_json,
        };
        let resp = self.post_data("setStickerSetThumbnail", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to upload a file with a sticker for later use in the createNewStickerSet and addStickerToSet methods (the file can be used multiple times). Returns the uploaded File on success."]
    pub async fn upload_sticker_file<'a>(
        &self,
        user_id: i64,
        sticker: FileData,
        sticker_format: &'a str,
    ) -> BotResult<File> {
        let data = Form::new();
        let (data, sticker_json) = sticker.to_form(data, "sticker".to_owned())?;
        let form = UploadStickerFileOpts {
            user_id: user_id,
            sticker: sticker_json,
            sticker_format: sticker_format,
        };
        let resp = self.post_data("uploadStickerFile", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub async fn edit_message_reply_markup<'a>(
        &self,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<MessageBool> {
        let form = EditMessageReplyMarkupOpts {
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("editMessageReplyMarkup", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty."]
    pub async fn get_webhook_info<'a>(&self) -> BotResult<WebhookInfo> {
        let resp = self.post_empty("getWebhookInfo").await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success."]
    pub async fn set_chat_permissions<'a>(
        &self,
        chat_id: i64,
        permissions: &'a ChatPermissions,
        use_independent_chat_permissions: Option<bool>,
    ) -> BotResult<bool> {
        let form = SetChatPermissionsOpts {
            chat_id: chat_id,
            permissions: serde_json::to_string(&permissions)?,
            use_independent_chat_permissions: use_independent_chat_permissions,
        };
        let resp = self.post("setChatPermissions", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success."]
    pub async fn unpin_all_forum_topic_messages<'a>(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> BotResult<bool> {
        let form = UnpinAllForumTopicMessagesOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
        };
        let resp = self.post("unpinAllForumTopicMessages", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success."]
    pub async fn get_chat<'a>(&self, chat_id: i64) -> BotResult<Chat> {
        let form = GetChatOpts { chat_id: chat_id };
        let resp = self.post("getChat", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success."]
    pub async fn set_sticker_emoji_list<'a>(
        &self,
        sticker: &'a str,
        emoji_list: &'a Vec<String>,
    ) -> BotResult<bool> {
        let form = SetStickerEmojiListOpts {
            sticker: sticker,
            emoji_list: serde_json::to_string(&emoji_list)?,
        };
        let resp = self.post("setStickerEmojiList", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to forward messages of any kind. Service messages can't be forwarded. On success, the sent Message is returned."]
    pub async fn forward_message<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        from_chat_id: i64,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_id: i64,
    ) -> BotResult<Message> {
        let form = ForwardMessageOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            from_chat_id: from_chat_id,
            disable_notification: disable_notification,
            protect_content: protect_content,
            message_id: message_id,
        };
        let resp = self.post("forwardMessage", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send phone contacts. On success, the sent Message is returned."]
    pub async fn send_contact<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        phone_number: &'a str,
        first_name: &'a str,
        last_name: Option<&'a str>,
        vcard: Option<&'a str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let form = SendContactOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            phone_number: phone_number,
            first_name: first_name,
            last_name: last_name,
            vcard: vcard,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendContact", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub async fn delete_chat_photo<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = DeleteChatPhotoOpts { chat_id: chat_id };
        let resp = self.post("deleteChatPhoto", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a ChatMember object on success."]
    pub async fn get_chat_member<'a>(&self, chat_id: i64, user_id: i64) -> BotResult<ChatMember> {
        let form = GetChatMemberOpts {
            chat_id: chat_id,
            user_id: user_id,
        };
        let resp = self.post("getChatMember", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically closed if it was open. Returns True on success."]
    pub async fn hide_general_forum_topic<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = HideGeneralForumTopicOpts { chat_id: chat_id };
        let resp = self.post("hideGeneralForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit text and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub async fn edit_message_text<'a>(
        &self,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
        text: &'a str,
        parse_mode: Option<&'a str>,
        entities: Option<&'a Vec<MessageEntity>>,
        disable_web_page_preview: Option<bool>,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<MessageBool> {
        let form = EditMessageTextOpts {
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
            text: text,
            parse_mode: parse_mode,
            entities: if let Some(entities) = entities {
                Some(serde_json::to_string(&entities)?)
            } else {
                None
            },
            disable_web_page_preview: disable_web_page_preview,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("editMessageText", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success."]
    pub async fn set_sticker_position_in_set<'a>(
        &self,
        sticker: &'a str,
        position: i64,
    ) -> BotResult<bool> {
        let form = SetStickerPositionInSetOpts {
            sticker: sticker,
            position: position,
        };
        let resp = self.post("setStickerPositionInSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future."]
    pub async fn send_video<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        video: FileData,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        thumbnail: Option<FileData>,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        has_spoiler: Option<bool>,
        supports_streaming: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, video_json) = video.to_form(data, "video".to_owned())?;
        let (data, thumbnail_json) = if let Some(thumbnail) = thumbnail {
            let (data, thumbnail_json) = thumbnail.to_form(data, "thumbnail".to_owned())?;
            (data, Some(thumbnail_json))
        } else {
            (data, None)
        };
        let form = SendVideoOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            video: video_json,
            duration: duration,
            width: width,
            height: height,
            thumbnail: thumbnail_json,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            has_spoiler: has_spoiler,
            supports_streaming: supports_streaming,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendVideo", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send answers to an inline query. On success, True is returned.No more than 50 results per query are allowed."]
    pub async fn answer_inline_query<'a>(
        &self,
        inline_query_id: &'a str,
        results: &'a Vec<InlineQueryResult>,
        cache_time: Option<i64>,
        is_personal: Option<bool>,
        next_offset: Option<&'a str>,
        button: Option<&'a InlineQueryResultsButton>,
    ) -> BotResult<bool> {
        let form = AnswerInlineQueryOpts {
            inline_query_id: inline_query_id,
            results: serde_json::to_string(&results)?,
            cache_time: cache_time,
            is_personal: is_personal,
            next_offset: next_offset,
            button: if let Some(button) = button {
                Some(serde_json::to_string(&button)?)
            } else {
                None
            },
        };
        let resp = self.post("answerInlineQuery", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub async fn ban_chat_sender_chat<'a>(
        &self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> BotResult<bool> {
        let form = BanChatSenderChatOpts {
            chat_id: chat_id,
            sender_chat_id: sender_chat_id,
        };
        let resp = self.post("banChatSenderChat", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass False for all boolean parameters to demote a user. Returns True on success."]
    pub async fn promote_chat_member<'a>(
        &self,
        chat_id: i64,
        user_id: i64,
        is_anonymous: Option<bool>,
        can_manage_chat: Option<bool>,
        can_delete_messages: Option<bool>,
        can_manage_video_chats: Option<bool>,
        can_restrict_members: Option<bool>,
        can_promote_members: Option<bool>,
        can_change_info: Option<bool>,
        can_invite_users: Option<bool>,
        can_post_messages: Option<bool>,
        can_edit_messages: Option<bool>,
        can_pin_messages: Option<bool>,
        can_post_stories: Option<bool>,
        can_edit_stories: Option<bool>,
        can_delete_stories: Option<bool>,
        can_manage_topics: Option<bool>,
    ) -> BotResult<bool> {
        let form = PromoteChatMemberOpts {
            chat_id: chat_id,
            user_id: user_id,
            is_anonymous: is_anonymous,
            can_manage_chat: can_manage_chat,
            can_delete_messages: can_delete_messages,
            can_manage_video_chats: can_manage_video_chats,
            can_restrict_members: can_restrict_members,
            can_promote_members: can_promote_members,
            can_change_info: can_change_info,
            can_invite_users: can_invite_users,
            can_post_messages: can_post_messages,
            can_edit_messages: can_edit_messages,
            can_pin_messages: can_pin_messages,
            can_post_stories: can_post_stories,
            can_edit_stories: can_edit_stories,
            can_delete_stories: can_delete_stories,
            can_manage_topics: can_manage_topics,
        };
        let resp = self.post("promoteChatMember", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success."]
    pub async fn delete_my_commands<'a>(
        &self,
        scope: Option<&'a BotCommandScope>,
        language_code: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = DeleteMyCommandsOpts {
            scope: if let Some(scope) = scope {
                Some(serde_json::to_string(&scope)?)
            } else {
                None
            },
            language_code: language_code,
        };
        let resp = self.post("deleteMyCommands", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success."]
    pub async fn export_chat_invite_link<'a>(&self, chat_id: i64) -> BotResult<String> {
        let form = ExportChatInviteLinkOpts { chat_id: chat_id };
        let resp = self.post("exportChatInviteLink", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success."]
    pub async fn get_chat_menu_button<'a>(&self, chat_id: Option<i64>) -> BotResult<MenuButton> {
        let form = GetChatMenuButtonOpts { chat_id: chat_id };
        let resp = self.post("getChatMenuButton", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub async fn edit_message_live_location<'a>(
        &self,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
        latitude: ::ordered_float::OrderedFloat<f64>,
        longitude: ::ordered_float::OrderedFloat<f64>,
        horizontal_accuracy: Option<::ordered_float::OrderedFloat<f64>>,
        heading: Option<i64>,
        proximity_alert_radius: Option<i64>,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<MessageBool> {
        let form = EditMessageLiveLocationOpts {
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
            latitude: latitude,
            longitude: longitude,
            horizontal_accuracy: horizontal_accuracy,
            heading: heading,
            proximity_alert_radius: proximity_alert_radius,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("editMessageLiveLocation", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a message, including service messages, with the following limitations:- A message can only be deleted if it was sent less than 48 hours ago.- Service messages about a supergroup, channel, or forum topic creation can't be deleted.- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.- Bots can delete outgoing messages in private chats, groups, and supergroups.- Bots can delete incoming messages in private chats.- Bots granted can_post_messages permissions can delete outgoing messages in channels.- If the bot is an administrator of a group, it can delete any message there.- If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.Returns True on success."]
    pub async fn delete_message<'a>(&self, chat_id: i64, message_id: i64) -> BotResult<bool> {
        let form = DeleteMessageOpts {
            chat_id: chat_id,
            message_id: message_id,
        };
        let resp = self.post("deleteMessage", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success."]
    pub async fn set_sticker_mask_position<'a>(
        &self,
        sticker: &'a str,
        mask_position: Option<&'a MaskPosition>,
    ) -> BotResult<bool> {
        let form = SetStickerMaskPositionOpts {
            sticker: sticker,
            mask_position: if let Some(mask_position) = mask_position {
                Some(serde_json::to_string(&mask_position)?)
            } else {
                None
            },
        };
        let resp = self.post("setStickerMaskPosition", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub async fn set_chat_title<'a>(&self, chat_id: i64, title: &'a str) -> BotResult<bool> {
        let form = SetChatTitleOpts {
            chat_id: chat_id,
            title: title,
        };
        let resp = self.post("setChatTitle", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues."]
    pub async fn set_passport_data_errors<'a>(
        &self,
        user_id: i64,
        errors: &'a Vec<PassportElementError>,
    ) -> BotResult<bool> {
        let form = SetPassportDataErrorsOpts {
            user_id: user_id,
            errors: serde_json::to_string(&errors)?,
        };
        let resp = self.post("setPassportDataErrors", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters."]
    pub async fn close<'a>(&self) -> BotResult<bool> {
        let resp = self.post_empty("close").await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned."]
    pub async fn send_sticker<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        sticker: FileData,
        emoji: Option<&'a str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, sticker_json) = sticker.to_form(data, "sticker".to_owned())?;
        let form = SendStickerOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            sticker: sticker_json,
            emoji: emoji,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendSticker", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success."]
    pub async fn delete_webhook<'a>(&self, drop_pending_updates: Option<bool>) -> BotResult<bool> {
        let form = DeleteWebhookOpts {
            drop_pending_updates: drop_pending_updates,
        };
        let resp = self.post("deleteWebhook", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success."]
    pub async fn set_my_default_administrator_rights<'a>(
        &self,
        rights: Option<&'a ChatAdministratorRights>,
        for_channels: Option<bool>,
    ) -> BotResult<bool> {
        let form = SetMyDefaultAdministratorRightsOpts {
            rights: if let Some(rights) = rights {
                Some(serde_json::to_string(&rights)?)
            } else {
                None
            },
            for_channels: for_channels,
        };
        let resp = self.post("setMyDefaultAdministratorRights", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success."]
    pub async fn unpin_all_general_forum_topic_messages<'a>(
        &self,
        chat_id: i64,
    ) -> BotResult<bool> {
        let form = UnpinAllGeneralForumTopicMessagesOpts { chat_id: chat_id };
        let resp = self.post("unpinAllGeneralForumTopicMessages", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object."]
    pub async fn edit_chat_invite_link<'a>(
        &self,
        chat_id: i64,
        invite_link: &'a str,
        name: Option<&'a str>,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        creates_join_request: Option<bool>,
    ) -> BotResult<ChatInviteLink> {
        let form = EditChatInviteLinkOpts {
            chat_id: chat_id,
            invite_link: invite_link,
            name: name,
            expire_date: expire_date,
            member_limit: member_limit,
            creates_join_request: creates_join_request,
        };
        let resp = self.post("editChatInviteLink", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub async fn edit_message_caption<'a>(
        &self,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<MessageBool> {
        let form = EditMessageCaptionOpts {
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("editMessageCaption", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a sticker from a set created by the bot. Returns True on success."]
    pub async fn delete_sticker_from_set<'a>(&self, sticker: &'a str) -> BotResult<bool> {
        let form = DeleteStickerFromSetOpts { sticker: sticker };
        let resp = self.post("deleteStickerFromSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future."]
    pub async fn send_animation<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        animation: FileData,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        thumbnail: Option<FileData>,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        has_spoiler: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, animation_json) = animation.to_form(data, "animation".to_owned())?;
        let (data, thumbnail_json) = if let Some(thumbnail) = thumbnail {
            let (data, thumbnail_json) = thumbnail.to_form(data, "thumbnail".to_owned())?;
            (data, Some(thumbnail_json))
        } else {
            (data, None)
        };
        let form = SendAnimationOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            animation: animation_json,
            duration: duration,
            width: width,
            height: height,
            thumbnail: thumbnail_json,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            has_spoiler: has_spoiler,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendAnimation", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success."]
    pub async fn set_chat_sticker_set<'a>(
        &self,
        chat_id: i64,
        sticker_set_name: &'a str,
    ) -> BotResult<bool> {
        let form = SetChatStickerSetOpts {
            chat_id: chat_id,
            sticker_set_name: sticker_set_name,
        };
        let resp = self.post("setChatStickerSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive."]
    pub async fn send_chat_action<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        action: &'a str,
    ) -> BotResult<bool> {
        let form = SendChatActionOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            action: action,
        };
        let resp = self.post("sendChatAction", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.Note: This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received."]
    pub async fn get_file<'a>(&self, file_id: &'a str) -> BotResult<File> {
        let form = GetFileOpts { file_id: file_id };
        let resp = self.post("getFile", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to add a new sticker to a set created by the bot. The format of the added sticker must match the format of the other stickers in the set. Emoji sticker sets can have up to 200 stickers. Animated and video sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success."]
    pub async fn add_sticker_to_set<'a>(
        &self,
        user_id: i64,
        name: &'a str,
        sticker: &'a InputSticker,
    ) -> BotResult<bool> {
        let form = AddStickerToSetOpts {
            user_id: user_id,
            name: name,
            sticker: serde_json::to_string(&sticker)?,
        };
        let resp = self.post("addStickerToSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current bot name for the given user language. Returns BotName on success."]
    pub async fn get_my_name<'a>(&self, language_code: Option<&'a str>) -> BotResult<BotName> {
        let form = GetMyNameOpts {
            language_code: language_code,
        };
        let resp = self.post("getMyName", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the title of a created sticker set. Returns True on success."]
    pub async fn set_sticker_set_title<'a>(
        &self,
        name: &'a str,
        title: &'a str,
    ) -> BotResult<bool> {
        let form = SetStickerSetTitleOpts {
            name: name,
            title: title,
        };
        let resp = self.post("setStickerSetTitle", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success."]
    pub async fn close_forum_topic<'a>(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> BotResult<bool> {
        let form = CloseForumTopicOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
        };
        let resp = self.post("closeForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned."]
    pub async fn get_my_commands<'a>(
        &self,
        scope: Option<&'a BotCommandScope>,
        language_code: Option<&'a str>,
    ) -> BotResult<Vec<BotCommand>> {
        let form = GetMyCommandsOpts {
            scope: if let Some(scope) = scope {
                Some(serde_json::to_string(&scope)?)
            } else {
                None
            },
            language_code: language_code,
        };
        let resp = self.post("getMyCommands", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to copy messages of any kind. Service messages and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success."]
    pub async fn copy_message<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        from_chat_id: i64,
        message_id: i64,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<MessageId> {
        let form = CopyMessageOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            from_chat_id: from_chat_id,
            message_id: message_id,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("copyMessage", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success."]
    pub async fn unpin_all_chat_messages<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = UnpinAllChatMessagesOpts { chat_id: chat_id };
        let resp = self.post("unpinAllChatMessages", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the number of members in a chat. Returns Int on success."]
    pub async fn get_chat_member_count<'a>(&self, chat_id: i64) -> BotResult<i64> {
        let form = GetChatMemberCountOpts { chat_id: chat_id };
        let resp = self.post("getChatMemberCount", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent."]
    pub async fn answer_pre_checkout_query<'a>(
        &self,
        pre_checkout_query_id: &'a str,
        ok: bool,
        error_message: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = AnswerPreCheckoutQueryOpts {
            pre_checkout_query_id: pre_checkout_query_id,
            ok: ok,
            error_message: error_message,
        };
        let resp = self.post("answerPreCheckoutQuery", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object."]
    pub async fn create_chat_invite_link<'a>(
        &self,
        chat_id: i64,
        name: Option<&'a str>,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        creates_join_request: Option<bool>,
    ) -> BotResult<ChatInviteLink> {
        let form = CreateChatInviteLinkOpts {
            chat_id: chat_id,
            name: name,
            expire_date: expire_date,
            member_limit: member_limit,
            creates_join_request: creates_join_request,
        };
        let resp = self.post("createChatInviteLink", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success."]
    pub async fn edit_forum_topic<'a>(
        &self,
        chat_id: i64,
        message_thread_id: i64,
        name: Option<&'a str>,
        icon_custom_emoji_id: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = EditForumTopicOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            name: name,
            icon_custom_emoji_id: icon_custom_emoji_id,
        };
        let resp = self.post("editForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header \"X-Telegram-Bot-Api-Secret-Token\" with the secret token as content."]
    pub async fn set_webhook<'a>(
        &self,
        url: &'a str,
        certificate: Option<FileData>,
        ip_address: Option<&'a str>,
        max_connections: Option<i64>,
        allowed_updates: Option<&'a Vec<String>>,
        drop_pending_updates: Option<bool>,
        secret_token: Option<&'a str>,
    ) -> BotResult<bool> {
        let data = Form::new();
        let (data, certificate_json) = if let Some(certificate) = certificate {
            let (data, certificate_json) = certificate.to_form(data, "certificate".to_owned())?;
            (data, Some(certificate_json))
        } else {
            (data, None)
        };
        let form = SetWebhookOpts {
            url: url,
            certificate: certificate_json,
            ip_address: ip_address,
            max_connections: max_connections,
            allowed_updates: if let Some(allowed_updates) = allowed_updates {
                Some(serde_json::to_string(&allowed_updates)?)
            } else {
                None
            },
            drop_pending_updates: drop_pending_updates,
            secret_token: secret_token,
        };
        let resp = self.post_data("setWebhook", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters."]
    pub async fn log_out<'a>(&self) -> BotResult<bool> {
        let resp = self.post_empty("logOut").await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object."]
    pub async fn revoke_chat_invite_link<'a>(
        &self,
        chat_id: i64,
        invite_link: &'a str,
    ) -> BotResult<ChatInviteLink> {
        let form = RevokeChatInviteLinkOpts {
            chat_id: chat_id,
            invite_link: invite_link,
        };
        let resp = self.post("revokeChatInviteLink", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success."]
    pub async fn reopen_forum_topic<'a>(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> BotResult<bool> {
        let form = ReopenForumTopicOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
        };
        let resp = self.post("reopenForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success."]
    pub async fn get_my_short_description<'a>(
        &self,
        language_code: Option<&'a str>,
    ) -> BotResult<BotShortDescription> {
        let form = GetMyShortDescriptionOpts {
            language_code: language_code,
        };
        let resp = self.post("getMyShortDescription", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success."]
    pub async fn get_my_default_administrator_rights<'a>(
        &self,
        for_channels: Option<bool>,
    ) -> BotResult<ChatAdministratorRights> {
        let form = GetMyDefaultAdministratorRightsOpts {
            for_channels: for_channels,
        };
        let resp = self.post("getMyDefaultAdministratorRights", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success."]
    pub async fn set_sticker_keywords<'a>(
        &self,
        sticker: &'a str,
        keywords: Option<&'a Vec<String>>,
    ) -> BotResult<bool> {
        let form = SetStickerKeywordsOpts {
            sticker: sticker,
            keywords: if let Some(keywords) = keywords {
                Some(serde_json::to_string(&keywords)?)
            } else {
                None
            },
        };
        let resp = self.post("setStickerKeywords", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success."]
    pub async fn set_chat_menu_button<'a>(
        &self,
        chat_id: Option<i64>,
        menu_button: Option<&'a MenuButton>,
    ) -> BotResult<bool> {
        let form = SetChatMenuButtonOpts {
            chat_id: chat_id,
            menu_button: if let Some(menu_button) = menu_button {
                Some(serde_json::to_string(&menu_button)?)
            } else {
                None
            },
        };
        let resp = self.post("setChatMenuButton", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub async fn edit_message_media<'a>(
        &self,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
        media: &'a InputMedia,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<MessageBool> {
        let form = EditMessageMediaOpts {
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
            media: serde_json::to_string(&media)?,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("editMessageMedia", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically unhidden if it was hidden. Returns True on success."]
    pub async fn reopen_general_forum_topic<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = ReopenGeneralForumTopicOpts { chat_id: chat_id };
        let resp = self.post("reopenGeneralForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects."]
    pub async fn get_game_high_scores<'a>(
        &self,
        user_id: i64,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
    ) -> BotResult<Vec<GameHighScore>> {
        let form = GetGameHighScoresOpts {
            user_id: user_id,
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
        };
        let resp = self.post("getGameHighScores", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns information about the created topic as a ForumTopic object."]
    pub async fn create_forum_topic<'a>(
        &self,
        chat_id: i64,
        name: &'a str,
        icon_color: Option<i64>,
        icon_custom_emoji_id: Option<&'a str>,
    ) -> BotResult<ForumTopic> {
        let form = CreateForumTopicOpts {
            chat_id: chat_id,
            name: name,
            icon_color: icon_color,
            icon_custom_emoji_id: icon_custom_emoji_id,
        };
        let resp = self.post("createForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send invoices. On success, the sent Message is returned."]
    pub async fn send_invoice<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        currency: &'a str,
        prices: &'a Vec<LabeledPrice>,
        max_tip_amount: Option<i64>,
        suggested_tip_amounts: Option<Vec<i64>>,
        start_parameter: Option<&'a str>,
        provider_data: Option<&'a str>,
        photo_url: Option<&'a str>,
        photo_size: Option<i64>,
        photo_width: Option<i64>,
        photo_height: Option<i64>,
        need_name: Option<bool>,
        need_phone_number: Option<bool>,
        need_email: Option<bool>,
        need_shipping_address: Option<bool>,
        send_phone_number_to_provider: Option<bool>,
        send_email_to_provider: Option<bool>,
        is_flexible: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<Message> {
        let form = SendInvoiceOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            title: title,
            description: description,
            payload: payload,
            provider_token: provider_token,
            currency: currency,
            prices: serde_json::to_string(&prices)?,
            max_tip_amount: max_tip_amount,
            suggested_tip_amounts: if let Some(suggested_tip_amounts) = suggested_tip_amounts {
                Some(serde_json::to_string(&suggested_tip_amounts)?)
            } else {
                None
            },
            start_parameter: start_parameter,
            provider_data: provider_data,
            photo_url: photo_url,
            photo_size: photo_size,
            photo_width: photo_width,
            photo_height: photo_height,
            need_name: need_name,
            need_phone_number: need_phone_number,
            need_email: need_email,
            need_shipping_address: need_shipping_address,
            send_phone_number_to_provider: send_phone_number_to_provider,
            send_email_to_provider: send_email_to_provider,
            is_flexible: is_flexible,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendInvoice", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method for your bot to leave a group, supergroup or channel. Returns True on success."]
    pub async fn leave_chat<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = LeaveChatOpts { chat_id: chat_id };
        let resp = self.post("leaveChat", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success."]
    pub async fn delete_chat_sticker_set<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = DeleteChatStickerSetOpts { chat_id: chat_id };
        let resp = self.post("deleteChatStickerSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get a sticker set. On success, a StickerSet object is returned."]
    pub async fn get_sticker_set<'a>(&self, name: &'a str) -> BotResult<StickerSet> {
        let form = GetStickerSetOpts { name: name };
        let resp = self.post("getStickerSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current bot description for the given user language. Returns BotDescription on success."]
    pub async fn get_my_description<'a>(
        &self,
        language_code: Option<&'a str>,
    ) -> BotResult<BotDescription> {
        let form = GetMyDescriptionOpts {
            language_code: language_code,
        };
        let resp = self.post("getMyDescription", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send photos. On success, the sent Message is returned."]
    pub async fn send_photo<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        photo: FileData,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        has_spoiler: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, photo_json) = photo.to_form(data, "photo".to_owned())?;
        let form = SendPhotoOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            photo: photo_json,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            has_spoiler: has_spoiler,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendPhoto", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success."]
    pub async fn close_general_forum_topic<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = CloseGeneralForumTopicOpts { chat_id: chat_id };
        let resp = self.post("closeGeneralForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub async fn set_chat_description<'a>(
        &self,
        chat_id: i64,
        description: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = SetChatDescriptionOpts {
            chat_id: chat_id,
            description: description,
        };
        let resp = self.post("setChatDescription", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects."]
    pub async fn get_chat_administrators<'a>(&self, chat_id: i64) -> BotResult<Vec<ChatMember>> {
        let form = GetChatAdministratorsOpts { chat_id: chat_id };
        let resp = self.post("getChatAdministrators", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of Sticker objects."]
    pub async fn get_forum_topic_icon_stickers<'a>(&self) -> BotResult<Vec<Sticker>> {
        let resp = self.post_empty("getForumTopicIconStickers").await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success."]
    pub async fn unhide_general_forum_topic<'a>(&self, chat_id: i64) -> BotResult<bool> {
        let form = UnhideGeneralForumTopicOpts { chat_id: chat_id };
        let resp = self.post("unhideGeneralForumTopic", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned."]
    pub async fn stop_poll<'a>(
        &self,
        chat_id: i64,
        message_id: i64,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<Poll> {
        let form = StopPollOpts {
            chat_id: chat_id,
            message_id: message_id,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("stopPoll", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub async fn ban_chat_member<'a>(
        &self,
        chat_id: i64,
        user_id: i64,
        until_date: Option<i64>,
        revoke_messages: Option<bool>,
    ) -> BotResult<bool> {
        let form = BanChatMemberOpts {
            chat_id: chat_id,
            user_id: user_id,
            until_date: until_date,
            revoke_messages: revoke_messages,
        };
        let resp = self.post("banChatMember", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send information about a venue. On success, the sent Message is returned."]
    pub async fn send_venue<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        latitude: ::ordered_float::OrderedFloat<f64>,
        longitude: ::ordered_float::OrderedFloat<f64>,
        title: &'a str,
        address: &'a str,
        foursquare_id: Option<&'a str>,
        foursquare_type: Option<&'a str>,
        google_place_id: Option<&'a str>,
        google_place_type: Option<&'a str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let form = SendVenueOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            latitude: latitude,
            longitude: longitude,
            title: title,
            address: address,
            foursquare_id: foursquare_id,
            foursquare_type: foursquare_type,
            google_place_id: google_place_id,
            google_place_type: google_place_type,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendVenue", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success."]
    pub async fn decline_chat_join_request<'a>(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> BotResult<bool> {
        let form = DeclineChatJoinRequestOpts {
            chat_id: chat_id,
            user_id: user_id,
        };
        let resp = self.post("declineChatJoinRequest", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub async fn set_chat_photo<'a>(&self, chat_id: i64, photo: FileData) -> BotResult<bool> {
        let data = Form::new();
        let (data, photo_json) = photo.to_form(data, "photo".to_owned())?;
        let form = SetChatPhotoOpts {
            chat_id: chat_id,
            photo: photo_json,
        };
        let resp = self.post_data("setChatPhoto", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create a link for an invoice. Returns the created invoice link as String on success."]
    pub async fn create_invoice_link<'a>(
        &self,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        currency: &'a str,
        prices: &'a Vec<LabeledPrice>,
        max_tip_amount: Option<i64>,
        suggested_tip_amounts: Option<Vec<i64>>,
        provider_data: Option<&'a str>,
        photo_url: Option<&'a str>,
        photo_size: Option<i64>,
        photo_width: Option<i64>,
        photo_height: Option<i64>,
        need_name: Option<bool>,
        need_phone_number: Option<bool>,
        need_email: Option<bool>,
        need_shipping_address: Option<bool>,
        send_phone_number_to_provider: Option<bool>,
        send_email_to_provider: Option<bool>,
        is_flexible: Option<bool>,
    ) -> BotResult<String> {
        let form = CreateInvoiceLinkOpts {
            title: title,
            description: description,
            payload: payload,
            provider_token: provider_token,
            currency: currency,
            prices: serde_json::to_string(&prices)?,
            max_tip_amount: max_tip_amount,
            suggested_tip_amounts: if let Some(suggested_tip_amounts) = suggested_tip_amounts {
                Some(serde_json::to_string(&suggested_tip_amounts)?)
            } else {
                None
            },
            provider_data: provider_data,
            photo_url: photo_url,
            photo_size: photo_size,
            photo_width: photo_width,
            photo_height: photo_height,
            need_name: need_name,
            need_phone_number: need_phone_number,
            need_email: need_email,
            need_shipping_address: need_shipping_address,
            send_phone_number_to_provider: send_phone_number_to_provider,
            send_email_to_provider: send_email_to_provider,
            is_flexible: is_flexible,
        };
        let resp = self.post("createInvoiceLink", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub async fn stop_message_live_location<'a>(
        &self,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<MessageBool> {
        let form = StopMessageLiveLocationOpts {
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("stopMessageLiveLocation", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send a game. On success, the sent Message is returned."]
    pub async fn send_game<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        game_short_name: &'a str,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a InlineKeyboardMarkup>,
    ) -> BotResult<Message> {
        let form = SendGameOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            game_short_name: game_short_name,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendGame", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to unban a previously banned user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success."]
    pub async fn unban_chat_member<'a>(
        &self,
        chat_id: i64,
        user_id: i64,
        only_if_banned: Option<bool>,
    ) -> BotResult<bool> {
        let form = UnbanChatMemberOpts {
            chat_id: chat_id,
            user_id: user_id,
            only_if_banned: only_if_banned,
        };
        let resp = self.post("unbanChatMember", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future."]
    pub async fn send_voice<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        voice: FileData,
        caption: Option<&'a str>,
        parse_mode: Option<&'a str>,
        caption_entities: Option<&'a Vec<MessageEntity>>,
        duration: Option<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let data = Form::new();
        let (data, voice_json) = voice.to_form(data, "voice".to_owned())?;
        let form = SendVoiceOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            voice: voice_json,
            caption: caption,
            parse_mode: parse_mode,
            caption_entities: if let Some(caption_entities) = caption_entities {
                Some(serde_json::to_string(&caption_entities)?)
            } else {
                None
            },
            duration: duration,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post_data("sendVoice", form, data).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned."]
    pub async fn send_dice<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        emoji: Option<&'a str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let form = SendDiceOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            emoji: emoji,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendDice", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned."]
    pub async fn answer_callback_query<'a>(
        &self,
        callback_query_id: &'a str,
        text: Option<&'a str>,
        show_alert: Option<bool>,
        url: Option<&'a str>,
        cache_time: Option<i64>,
    ) -> BotResult<bool> {
        let form = AnswerCallbackQueryOpts {
            callback_query_id: callback_query_id,
            text: text,
            show_alert: show_alert,
            url: url,
            cache_time: cache_time,
        };
        let resp = self.post("answerCallbackQuery", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub async fn unban_chat_sender_chat<'a>(
        &self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> BotResult<bool> {
        let form = UnbanChatSenderChatOpts {
            chat_id: chat_id,
            sender_chat_id: sender_chat_id,
        };
        let resp = self.post("unbanChatSenderChat", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's name. Returns True on success."]
    pub async fn set_my_name<'a>(
        &self,
        name: Option<&'a str>,
        language_code: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = SetMyNameOpts {
            name: name,
            language_code: language_code,
        };
        let resp = self.post("setMyName", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a sticker set that was created by the bot. Returns True on success."]
    pub async fn delete_sticker_set<'a>(&self, name: &'a str) -> BotResult<bool> {
        let form = DeleteStickerSetOpts { name: name };
        let resp = self.post("deleteStickerSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send a native poll. On success, the sent Message is returned."]
    pub async fn send_poll<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        question: &'a str,
        options: &'a Vec<String>,
        is_anonymous: Option<bool>,
        tg_type: Option<&'a str>,
        allows_multiple_answers: Option<bool>,
        correct_option_id: Option<i64>,
        explanation: Option<&'a str>,
        explanation_parse_mode: Option<&'a str>,
        explanation_entities: Option<&'a Vec<MessageEntity>>,
        open_period: Option<i64>,
        close_date: Option<i64>,
        is_closed: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let form = SendPollOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            question: question,
            options: serde_json::to_string(&options)?,
            is_anonymous: is_anonymous,
            tg_type: tg_type,
            allows_multiple_answers: allows_multiple_answers,
            correct_option_id: correct_option_id,
            explanation: explanation,
            explanation_parse_mode: explanation_parse_mode,
            explanation_entities: if let Some(explanation_entities) = explanation_entities {
                Some(serde_json::to_string(&explanation_entities)?)
            } else {
                None
            },
            open_period: open_period,
            close_date: close_date,
            is_closed: is_closed,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendPoll", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object."]
    pub async fn get_user_profile_photos<'a>(
        &self,
        user_id: i64,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> BotResult<UserProfilePhotos> {
        let form = GetUserProfilePhotosOpts {
            user_id: user_id,
            offset: offset,
            limit: limit,
        };
        let resp = self.post("getUserProfilePhotos", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns True on success."]
    pub async fn set_my_short_description<'a>(
        &self,
        short_description: Option<&'a str>,
        language_code: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = SetMyShortDescriptionOpts {
            short_description: short_description,
            language_code: language_code,
        };
        let resp = self.post("setMyShortDescription", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns True on success."]
    pub async fn create_new_sticker_set<'a>(
        &self,
        user_id: i64,
        name: &'a str,
        title: &'a str,
        stickers: &'a Vec<InputSticker>,
        sticker_format: &'a str,
        sticker_type: Option<&'a str>,
        needs_repainting: Option<bool>,
    ) -> BotResult<bool> {
        let form = CreateNewStickerSetOpts {
            user_id: user_id,
            name: name,
            title: title,
            stickers: serde_json::to_string(&stickers)?,
            sticker_format: sticker_format,
            sticker_type: sticker_type,
            needs_repainting: needs_repainting,
        };
        let resp = self.post("createNewStickerSet", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned."]
    pub async fn answer_shipping_query<'a>(
        &self,
        shipping_query_id: &'a str,
        ok: bool,
        shipping_options: Option<&'a Vec<ShippingOption>>,
        error_message: Option<&'a str>,
    ) -> BotResult<bool> {
        let form = AnswerShippingQueryOpts {
            shipping_query_id: shipping_query_id,
            ok: ok,
            shipping_options: if let Some(shipping_options) = shipping_options {
                Some(serde_json::to_string(&shipping_options)?)
            } else {
                None
            },
            error_message: error_message,
        };
        let resp = self.post("answerShippingQuery", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send text messages. On success, the sent Message is returned."]
    pub async fn send_message<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        text: &'a str,
        parse_mode: Option<&'a str>,
        entities: Option<&'a Vec<MessageEntity>>,
        disable_web_page_preview: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let form = SendMessageOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            text: text,
            parse_mode: parse_mode,
            entities: if let Some(entities) = entities {
                Some(serde_json::to_string(&entities)?)
            } else {
                None
            },
            disable_web_page_preview: disable_web_page_preview,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendMessage", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects."]
    pub async fn get_updates<'a>(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
        timeout: Option<i64>,
        allowed_updates: Option<&'a Vec<String>>,
    ) -> BotResult<Vec<Update>> {
        let form = GetUpdatesOpts {
            offset: offset,
            limit: limit,
            timeout: timeout,
            allowed_updates: if let Some(allowed_updates) = allowed_updates {
                Some(serde_json::to_string(&allowed_updates)?)
            } else {
                None
            },
        };
        let resp = self.post("getUpdates", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send point on the map. On success, the sent Message is returned."]
    pub async fn send_location<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        latitude: ::ordered_float::OrderedFloat<f64>,
        longitude: ::ordered_float::OrderedFloat<f64>,
        horizontal_accuracy: Option<::ordered_float::OrderedFloat<f64>>,
        live_period: Option<i64>,
        heading: Option<i64>,
        proximity_alert_radius: Option<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<&'a EReplyMarkup>,
    ) -> BotResult<Message> {
        let form = SendLocationOpts {
            chat_id: chat_id,
            message_thread_id: message_thread_id,
            latitude: latitude,
            longitude: longitude,
            horizontal_accuracy: horizontal_accuracy,
            live_period: live_period,
            heading: heading,
            proximity_alert_radius: proximity_alert_radius,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: if let Some(reply_markup) = reply_markup {
                Some(serde_json::to_string(&reply_markup)?)
            } else {
                None
            },
        };
        let resp = self.post("sendLocation", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False."]
    pub async fn set_game_score<'a>(
        &self,
        user_id: i64,
        score: i64,
        force: Option<bool>,
        disable_edit_message: Option<bool>,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&'a str>,
    ) -> BotResult<MessageBool> {
        let form = SetGameScoreOpts {
            user_id: user_id,
            score: score,
            force: force,
            disable_edit_message: disable_edit_message,
            chat_id: chat_id,
            message_id: message_id,
            inline_message_id: inline_message_id,
        };
        let resp = self.post("setGameScore", form).await?;
        if resp.ok {
            let res = resp.result.unwrap_or_default();
            let resp = serde_json::from_value(res)?;
            Ok(resp)
        } else {
            Err(ApiError::from_response(resp))
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success."]
    pub fn build_approve_chat_join_request<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
    ) -> CallApproveChatJoinRequest<'a> {
        CallApproveChatJoinRequest {
            bot: self,
            chat_id,
            user_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_delete_messages administrator rights. Returns True on success."]
    pub fn build_delete_forum_topic<'a>(
        &'a self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> CallDeleteForumTopic<'a> {
        CallDeleteForumTopic {
            bot: self,
            chat_id,
            message_thread_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned."]
    pub fn build_send_video_note<'a>(
        &'a self,
        chat_id: i64,
        video_note: FileData,
    ) -> CallSendVideoNote<'a> {
        CallSendVideoNote {
            bot: self,
            chat_id,
            video_note,
            message_thread_id: None,
            duration: None,
            length: None,
            thumbnail: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success."]
    pub fn build_pin_chat_message<'a>(
        &'a self,
        chat_id: i64,
        message_id: i64,
    ) -> CallPinChatMessage<'a> {
        CallPinChatMessage {
            bot: self,
            chat_id,
            message_id,
            disable_notification: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.For sending voice messages, use the sendVoice method instead."]
    pub fn build_send_audio<'a>(&'a self, chat_id: i64, audio: FileData) -> CallSendAudio<'a> {
        CallSendAudio {
            bot: self,
            chat_id,
            audio,
            message_thread_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
            thumbnail: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns True on success."]
    pub fn build_set_my_commands<'a>(
        &'a self,
        commands: &'a Vec<BotCommand>,
    ) -> CallSetMyCommands<'a> {
        CallSetMyCommands {
            bot: self,
            commands,
            scope: None,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object."]
    pub fn build_get_me<'a>(&'a self) -> CallGetMe<'a> {
        CallGetMe { bot: self }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects."]
    pub fn build_get_custom_emoji_stickers<'a>(
        &'a self,
        custom_emoji_ids: &'a Vec<String>,
    ) -> CallGetCustomEmojiStickers<'a> {
        CallGetCustomEmojiStickers {
            bot: self,
            custom_emoji_ids,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned."]
    pub fn build_answer_web_app_query<'a>(
        &'a self,
        web_app_query_id: &'a str,
        result: &'a InlineQueryResult,
    ) -> CallAnswerWebAppQuery<'a> {
        CallAnswerWebAppQuery {
            bot: self,
            web_app_query_id,
            result,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success."]
    pub fn build_restrict_chat_member<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
        permissions: &'a ChatPermissions,
    ) -> CallRestrictChatMember<'a> {
        CallRestrictChatMember {
            bot: self,
            chat_id,
            user_id,
            permissions,
            use_independent_chat_permissions: None,
            until_date: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success."]
    pub fn build_unpin_chat_message<'a>(&'a self, chat_id: i64) -> CallUnpinChatMessage<'a> {
        CallUnpinChatMessage {
            bot: self,
            chat_id,
            message_id: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success."]
    pub fn build_set_chat_administrator_custom_title<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
        custom_title: &'a str,
    ) -> CallSetChatAdministratorCustomTitle<'a> {
        CallSetChatAdministratorCustomTitle {
            bot: self,
            chat_id,
            user_id,
            custom_title,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future."]
    pub fn build_send_document<'a>(
        &'a self,
        chat_id: i64,
        document: FileData,
    ) -> CallSendDocument<'a> {
        CallSendDocument {
            bot: self,
            chat_id,
            document,
            message_thread_id: None,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights. Returns True on success."]
    pub fn build_edit_general_forum_topic<'a>(
        &'a self,
        chat_id: i64,
        name: &'a str,
    ) -> CallEditGeneralForumTopic<'a> {
        CallEditGeneralForumTopic {
            bot: self,
            chat_id,
            name,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success."]
    pub fn build_set_custom_emoji_sticker_set_thumbnail<'a>(
        &'a self,
        name: &'a str,
    ) -> CallSetCustomEmojiStickerSetThumbnail<'a> {
        CallSetCustomEmojiStickerSetThumbnail {
            bot: self,
            name,
            custom_emoji_id: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned."]
    pub fn build_send_media_group<'a>(
        &'a self,
        chat_id: i64,
        media: &'a Vec<EMedia>,
    ) -> CallSendMediaGroup<'a> {
        CallSendMediaGroup {
            bot: self,
            chat_id,
            media,
            message_thread_id: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns True on success."]
    pub fn build_set_my_description<'a>(&'a self) -> CallSetMyDescription<'a> {
        CallSetMyDescription {
            bot: self,
            description: None,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns True on success."]
    pub fn build_set_sticker_set_thumbnail<'a>(
        &'a self,
        name: &'a str,
        user_id: i64,
    ) -> CallSetStickerSetThumbnail<'a> {
        CallSetStickerSetThumbnail {
            bot: self,
            name,
            user_id,
            thumbnail: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to upload a file with a sticker for later use in the createNewStickerSet and addStickerToSet methods (the file can be used multiple times). Returns the uploaded File on success."]
    pub fn build_upload_sticker_file<'a>(
        &'a self,
        user_id: i64,
        sticker: FileData,
        sticker_format: &'a str,
    ) -> CallUploadStickerFile<'a> {
        CallUploadStickerFile {
            bot: self,
            user_id,
            sticker,
            sticker_format,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub fn build_edit_message_reply_markup<'a>(&'a self) -> CallEditMessageReplyMarkup<'a> {
        CallEditMessageReplyMarkup {
            bot: self,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty."]
    pub fn build_get_webhook_info<'a>(&'a self) -> CallGetWebhookInfo<'a> {
        CallGetWebhookInfo { bot: self }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success."]
    pub fn build_set_chat_permissions<'a>(
        &'a self,
        chat_id: i64,
        permissions: &'a ChatPermissions,
    ) -> CallSetChatPermissions<'a> {
        CallSetChatPermissions {
            bot: self,
            chat_id,
            permissions,
            use_independent_chat_permissions: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success."]
    pub fn build_unpin_all_forum_topic_messages<'a>(
        &'a self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> CallUnpinAllForumTopicMessages<'a> {
        CallUnpinAllForumTopicMessages {
            bot: self,
            chat_id,
            message_thread_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success."]
    pub fn build_get_chat<'a>(&'a self, chat_id: i64) -> CallGetChat<'a> {
        CallGetChat { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success."]
    pub fn build_set_sticker_emoji_list<'a>(
        &'a self,
        sticker: &'a str,
        emoji_list: &'a Vec<String>,
    ) -> CallSetStickerEmojiList<'a> {
        CallSetStickerEmojiList {
            bot: self,
            sticker,
            emoji_list,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to forward messages of any kind. Service messages can't be forwarded. On success, the sent Message is returned."]
    pub fn build_forward_message<'a>(
        &'a self,
        chat_id: i64,
        from_chat_id: i64,
        message_id: i64,
    ) -> CallForwardMessage<'a> {
        CallForwardMessage {
            bot: self,
            chat_id,
            from_chat_id,
            message_id,
            message_thread_id: None,
            disable_notification: None,
            protect_content: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send phone contacts. On success, the sent Message is returned."]
    pub fn build_send_contact<'a>(
        &'a self,
        chat_id: i64,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> CallSendContact<'a> {
        CallSendContact {
            bot: self,
            chat_id,
            phone_number,
            first_name,
            message_thread_id: None,
            last_name: None,
            vcard: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub fn build_delete_chat_photo<'a>(&'a self, chat_id: i64) -> CallDeleteChatPhoto<'a> {
        CallDeleteChatPhoto { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a ChatMember object on success."]
    pub fn build_get_chat_member<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
    ) -> CallGetChatMember<'a> {
        CallGetChatMember {
            bot: self,
            chat_id,
            user_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically closed if it was open. Returns True on success."]
    pub fn build_hide_general_forum_topic<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallHideGeneralForumTopic<'a> {
        CallHideGeneralForumTopic { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit text and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub fn build_edit_message_text<'a>(&'a self, text: &'a str) -> CallEditMessageText<'a> {
        CallEditMessageText {
            bot: self,
            text,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success."]
    pub fn build_set_sticker_position_in_set<'a>(
        &'a self,
        sticker: &'a str,
        position: i64,
    ) -> CallSetStickerPositionInSet<'a> {
        CallSetStickerPositionInSet {
            bot: self,
            sticker,
            position,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future."]
    pub fn build_send_video<'a>(&'a self, chat_id: i64, video: FileData) -> CallSendVideo<'a> {
        CallSendVideo {
            bot: self,
            chat_id,
            video,
            message_thread_id: None,
            duration: None,
            width: None,
            height: None,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            supports_streaming: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send answers to an inline query. On success, True is returned.No more than 50 results per query are allowed."]
    pub fn build_answer_inline_query<'a>(
        &'a self,
        inline_query_id: &'a str,
        results: &'a Vec<InlineQueryResult>,
    ) -> CallAnswerInlineQuery<'a> {
        CallAnswerInlineQuery {
            bot: self,
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            button: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub fn build_ban_chat_sender_chat<'a>(
        &'a self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> CallBanChatSenderChat<'a> {
        CallBanChatSenderChat {
            bot: self,
            chat_id,
            sender_chat_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass False for all boolean parameters to demote a user. Returns True on success."]
    pub fn build_promote_chat_member<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
    ) -> CallPromoteChatMember<'a> {
        CallPromoteChatMember {
            bot: self,
            chat_id,
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_delete_messages: None,
            can_manage_video_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            can_post_stories: None,
            can_edit_stories: None,
            can_delete_stories: None,
            can_manage_topics: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success."]
    pub fn build_delete_my_commands<'a>(&'a self) -> CallDeleteMyCommands<'a> {
        CallDeleteMyCommands {
            bot: self,
            scope: None,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success."]
    pub fn build_export_chat_invite_link<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallExportChatInviteLink<'a> {
        CallExportChatInviteLink { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success."]
    pub fn build_get_chat_menu_button<'a>(&'a self) -> CallGetChatMenuButton<'a> {
        CallGetChatMenuButton {
            bot: self,
            chat_id: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub fn build_edit_message_live_location<'a>(
        &'a self,
        latitude: ::ordered_float::OrderedFloat<f64>,
        longitude: ::ordered_float::OrderedFloat<f64>,
    ) -> CallEditMessageLiveLocation<'a> {
        CallEditMessageLiveLocation {
            bot: self,
            latitude,
            longitude,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a message, including service messages, with the following limitations:- A message can only be deleted if it was sent less than 48 hours ago.- Service messages about a supergroup, channel, or forum topic creation can't be deleted.- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.- Bots can delete outgoing messages in private chats, groups, and supergroups.- Bots can delete incoming messages in private chats.- Bots granted can_post_messages permissions can delete outgoing messages in channels.- If the bot is an administrator of a group, it can delete any message there.- If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.Returns True on success."]
    pub fn build_delete_message<'a>(
        &'a self,
        chat_id: i64,
        message_id: i64,
    ) -> CallDeleteMessage<'a> {
        CallDeleteMessage {
            bot: self,
            chat_id,
            message_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success."]
    pub fn build_set_sticker_mask_position<'a>(
        &'a self,
        sticker: &'a str,
    ) -> CallSetStickerMaskPosition<'a> {
        CallSetStickerMaskPosition {
            bot: self,
            sticker,
            mask_position: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub fn build_set_chat_title<'a>(
        &'a self,
        chat_id: i64,
        title: &'a str,
    ) -> CallSetChatTitle<'a> {
        CallSetChatTitle {
            bot: self,
            chat_id,
            title,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues."]
    pub fn build_set_passport_data_errors<'a>(
        &'a self,
        user_id: i64,
        errors: &'a Vec<PassportElementError>,
    ) -> CallSetPassportDataErrors<'a> {
        CallSetPassportDataErrors {
            bot: self,
            user_id,
            errors,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters."]
    pub fn build_close<'a>(&'a self) -> CallClose<'a> {
        CallClose { bot: self }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned."]
    pub fn build_send_sticker<'a>(
        &'a self,
        chat_id: i64,
        sticker: FileData,
    ) -> CallSendSticker<'a> {
        CallSendSticker {
            bot: self,
            chat_id,
            sticker,
            message_thread_id: None,
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success."]
    pub fn build_delete_webhook<'a>(&'a self) -> CallDeleteWebhook<'a> {
        CallDeleteWebhook {
            bot: self,
            drop_pending_updates: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success."]
    pub fn build_set_my_default_administrator_rights<'a>(
        &'a self,
    ) -> CallSetMyDefaultAdministratorRights<'a> {
        CallSetMyDefaultAdministratorRights {
            bot: self,
            rights: None,
            for_channels: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success."]
    pub fn build_unpin_all_general_forum_topic_messages<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallUnpinAllGeneralForumTopicMessages<'a> {
        CallUnpinAllGeneralForumTopicMessages { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object."]
    pub fn build_edit_chat_invite_link<'a>(
        &'a self,
        chat_id: i64,
        invite_link: &'a str,
    ) -> CallEditChatInviteLink<'a> {
        CallEditChatInviteLink {
            bot: self,
            chat_id,
            invite_link,
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub fn build_edit_message_caption<'a>(&'a self) -> CallEditMessageCaption<'a> {
        CallEditMessageCaption {
            bot: self,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a sticker from a set created by the bot. Returns True on success."]
    pub fn build_delete_sticker_from_set<'a>(
        &'a self,
        sticker: &'a str,
    ) -> CallDeleteStickerFromSet<'a> {
        CallDeleteStickerFromSet { bot: self, sticker }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future."]
    pub fn build_send_animation<'a>(
        &'a self,
        chat_id: i64,
        animation: FileData,
    ) -> CallSendAnimation<'a> {
        CallSendAnimation {
            bot: self,
            chat_id,
            animation,
            message_thread_id: None,
            duration: None,
            width: None,
            height: None,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success."]
    pub fn build_set_chat_sticker_set<'a>(
        &'a self,
        chat_id: i64,
        sticker_set_name: &'a str,
    ) -> CallSetChatStickerSet<'a> {
        CallSetChatStickerSet {
            bot: self,
            chat_id,
            sticker_set_name,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive."]
    pub fn build_send_chat_action<'a>(
        &'a self,
        chat_id: i64,
        action: &'a str,
    ) -> CallSendChatAction<'a> {
        CallSendChatAction {
            bot: self,
            chat_id,
            action,
            message_thread_id: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.Note: This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received."]
    pub fn build_get_file<'a>(&'a self, file_id: &'a str) -> CallGetFile<'a> {
        CallGetFile { bot: self, file_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to add a new sticker to a set created by the bot. The format of the added sticker must match the format of the other stickers in the set. Emoji sticker sets can have up to 200 stickers. Animated and video sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success."]
    pub fn build_add_sticker_to_set<'a>(
        &'a self,
        user_id: i64,
        name: &'a str,
        sticker: &'a InputSticker,
    ) -> CallAddStickerToSet<'a> {
        CallAddStickerToSet {
            bot: self,
            user_id,
            name,
            sticker,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current bot name for the given user language. Returns BotName on success."]
    pub fn build_get_my_name<'a>(&'a self) -> CallGetMyName<'a> {
        CallGetMyName {
            bot: self,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the title of a created sticker set. Returns True on success."]
    pub fn build_set_sticker_set_title<'a>(
        &'a self,
        name: &'a str,
        title: &'a str,
    ) -> CallSetStickerSetTitle<'a> {
        CallSetStickerSetTitle {
            bot: self,
            name,
            title,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success."]
    pub fn build_close_forum_topic<'a>(
        &'a self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> CallCloseForumTopic<'a> {
        CallCloseForumTopic {
            bot: self,
            chat_id,
            message_thread_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned."]
    pub fn build_get_my_commands<'a>(&'a self) -> CallGetMyCommands<'a> {
        CallGetMyCommands {
            bot: self,
            scope: None,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to copy messages of any kind. Service messages and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success."]
    pub fn build_copy_message<'a>(
        &'a self,
        chat_id: i64,
        from_chat_id: i64,
        message_id: i64,
    ) -> CallCopyMessage<'a> {
        CallCopyMessage {
            bot: self,
            chat_id,
            from_chat_id,
            message_id,
            message_thread_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success."]
    pub fn build_unpin_all_chat_messages<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallUnpinAllChatMessages<'a> {
        CallUnpinAllChatMessages { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the number of members in a chat. Returns Int on success."]
    pub fn build_get_chat_member_count<'a>(&'a self, chat_id: i64) -> CallGetChatMemberCount<'a> {
        CallGetChatMemberCount { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent."]
    pub fn build_answer_pre_checkout_query<'a>(
        &'a self,
        pre_checkout_query_id: &'a str,
        ok: bool,
    ) -> CallAnswerPreCheckoutQuery<'a> {
        CallAnswerPreCheckoutQuery {
            bot: self,
            pre_checkout_query_id,
            ok,
            error_message: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object."]
    pub fn build_create_chat_invite_link<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallCreateChatInviteLink<'a> {
        CallCreateChatInviteLink {
            bot: self,
            chat_id,
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success."]
    pub fn build_edit_forum_topic<'a>(
        &'a self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> CallEditForumTopic<'a> {
        CallEditForumTopic {
            bot: self,
            chat_id,
            message_thread_id,
            name: None,
            icon_custom_emoji_id: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header \"X-Telegram-Bot-Api-Secret-Token\" with the secret token as content."]
    pub fn build_set_webhook<'a>(&'a self, url: &'a str) -> CallSetWebhook<'a> {
        CallSetWebhook {
            bot: self,
            url,
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
            secret_token: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters."]
    pub fn build_log_out<'a>(&'a self) -> CallLogOut<'a> {
        CallLogOut { bot: self }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object."]
    pub fn build_revoke_chat_invite_link<'a>(
        &'a self,
        chat_id: i64,
        invite_link: &'a str,
    ) -> CallRevokeChatInviteLink<'a> {
        CallRevokeChatInviteLink {
            bot: self,
            chat_id,
            invite_link,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success."]
    pub fn build_reopen_forum_topic<'a>(
        &'a self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> CallReopenForumTopic<'a> {
        CallReopenForumTopic {
            bot: self,
            chat_id,
            message_thread_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success."]
    pub fn build_get_my_short_description<'a>(&'a self) -> CallGetMyShortDescription<'a> {
        CallGetMyShortDescription {
            bot: self,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success."]
    pub fn build_get_my_default_administrator_rights<'a>(
        &'a self,
    ) -> CallGetMyDefaultAdministratorRights<'a> {
        CallGetMyDefaultAdministratorRights {
            bot: self,
            for_channels: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success."]
    pub fn build_set_sticker_keywords<'a>(
        &'a self,
        sticker: &'a str,
    ) -> CallSetStickerKeywords<'a> {
        CallSetStickerKeywords {
            bot: self,
            sticker,
            keywords: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success."]
    pub fn build_set_chat_menu_button<'a>(&'a self) -> CallSetChatMenuButton<'a> {
        CallSetChatMenuButton {
            bot: self,
            chat_id: None,
            menu_button: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub fn build_edit_message_media<'a>(
        &'a self,
        media: &'a InputMedia,
    ) -> CallEditMessageMedia<'a> {
        CallEditMessageMedia {
            bot: self,
            media,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically unhidden if it was hidden. Returns True on success."]
    pub fn build_reopen_general_forum_topic<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallReopenGeneralForumTopic<'a> {
        CallReopenGeneralForumTopic { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects."]
    pub fn build_get_game_high_scores<'a>(&'a self, user_id: i64) -> CallGetGameHighScores<'a> {
        CallGetGameHighScores {
            bot: self,
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns information about the created topic as a ForumTopic object."]
    pub fn build_create_forum_topic<'a>(
        &'a self,
        chat_id: i64,
        name: &'a str,
    ) -> CallCreateForumTopic<'a> {
        CallCreateForumTopic {
            bot: self,
            chat_id,
            name,
            icon_color: None,
            icon_custom_emoji_id: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send invoices. On success, the sent Message is returned."]
    pub fn build_send_invoice<'a>(
        &'a self,
        chat_id: i64,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        currency: &'a str,
        prices: &'a Vec<LabeledPrice>,
    ) -> CallSendInvoice<'a> {
        CallSendInvoice {
            bot: self,
            chat_id,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            message_thread_id: None,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            start_parameter: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method for your bot to leave a group, supergroup or channel. Returns True on success."]
    pub fn build_leave_chat<'a>(&'a self, chat_id: i64) -> CallLeaveChat<'a> {
        CallLeaveChat { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success."]
    pub fn build_delete_chat_sticker_set<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallDeleteChatStickerSet<'a> {
        CallDeleteChatStickerSet { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get a sticker set. On success, a StickerSet object is returned."]
    pub fn build_get_sticker_set<'a>(&'a self, name: &'a str) -> CallGetStickerSet<'a> {
        CallGetStickerSet { bot: self, name }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get the current bot description for the given user language. Returns BotDescription on success."]
    pub fn build_get_my_description<'a>(&'a self) -> CallGetMyDescription<'a> {
        CallGetMyDescription {
            bot: self,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send photos. On success, the sent Message is returned."]
    pub fn build_send_photo<'a>(&'a self, chat_id: i64, photo: FileData) -> CallSendPhoto<'a> {
        CallSendPhoto {
            bot: self,
            chat_id,
            photo,
            message_thread_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success."]
    pub fn build_close_general_forum_topic<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallCloseGeneralForumTopic<'a> {
        CallCloseGeneralForumTopic { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub fn build_set_chat_description<'a>(&'a self, chat_id: i64) -> CallSetChatDescription<'a> {
        CallSetChatDescription {
            bot: self,
            chat_id,
            description: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects."]
    pub fn build_get_chat_administrators<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallGetChatAdministrators<'a> {
        CallGetChatAdministrators { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of Sticker objects."]
    pub fn build_get_forum_topic_icon_stickers<'a>(&'a self) -> CallGetForumTopicIconStickers<'a> {
        CallGetForumTopicIconStickers { bot: self }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success."]
    pub fn build_unhide_general_forum_topic<'a>(
        &'a self,
        chat_id: i64,
    ) -> CallUnhideGeneralForumTopic<'a> {
        CallUnhideGeneralForumTopic { bot: self, chat_id }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned."]
    pub fn build_stop_poll<'a>(&'a self, chat_id: i64, message_id: i64) -> CallStopPoll<'a> {
        CallStopPoll {
            bot: self,
            chat_id,
            message_id,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub fn build_ban_chat_member<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
    ) -> CallBanChatMember<'a> {
        CallBanChatMember {
            bot: self,
            chat_id,
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send information about a venue. On success, the sent Message is returned."]
    pub fn build_send_venue<'a>(
        &'a self,
        chat_id: i64,
        latitude: ::ordered_float::OrderedFloat<f64>,
        longitude: ::ordered_float::OrderedFloat<f64>,
        title: &'a str,
        address: &'a str,
    ) -> CallSendVenue<'a> {
        CallSendVenue {
            bot: self,
            chat_id,
            latitude,
            longitude,
            title,
            address,
            message_thread_id: None,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success."]
    pub fn build_decline_chat_join_request<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
    ) -> CallDeclineChatJoinRequest<'a> {
        CallDeclineChatJoinRequest {
            bot: self,
            chat_id,
            user_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub fn build_set_chat_photo<'a>(
        &'a self,
        chat_id: i64,
        photo: FileData,
    ) -> CallSetChatPhoto<'a> {
        CallSetChatPhoto {
            bot: self,
            chat_id,
            photo,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create a link for an invoice. Returns the created invoice link as String on success."]
    pub fn build_create_invoice_link<'a>(
        &'a self,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        currency: &'a str,
        prices: &'a Vec<LabeledPrice>,
    ) -> CallCreateInvoiceLink<'a> {
        CallCreateInvoiceLink {
            bot: self,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned."]
    pub fn build_stop_message_live_location<'a>(&'a self) -> CallStopMessageLiveLocation<'a> {
        CallStopMessageLiveLocation {
            bot: self,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send a game. On success, the sent Message is returned."]
    pub fn build_send_game<'a>(
        &'a self,
        chat_id: i64,
        game_short_name: &'a str,
    ) -> CallSendGame<'a> {
        CallSendGame {
            bot: self,
            chat_id,
            game_short_name,
            message_thread_id: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to unban a previously banned user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success."]
    pub fn build_unban_chat_member<'a>(
        &'a self,
        chat_id: i64,
        user_id: i64,
    ) -> CallUnbanChatMember<'a> {
        CallUnbanChatMember {
            bot: self,
            chat_id,
            user_id,
            only_if_banned: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future."]
    pub fn build_send_voice<'a>(&'a self, chat_id: i64, voice: FileData) -> CallSendVoice<'a> {
        CallSendVoice {
            bot: self,
            chat_id,
            voice,
            message_thread_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned."]
    pub fn build_send_dice<'a>(&'a self, chat_id: i64) -> CallSendDice<'a> {
        CallSendDice {
            bot: self,
            chat_id,
            message_thread_id: None,
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned."]
    pub fn build_answer_callback_query<'a>(
        &'a self,
        callback_query_id: &'a str,
    ) -> CallAnswerCallbackQuery<'a> {
        CallAnswerCallbackQuery {
            bot: self,
            callback_query_id,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success."]
    pub fn build_unban_chat_sender_chat<'a>(
        &'a self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> CallUnbanChatSenderChat<'a> {
        CallUnbanChatSenderChat {
            bot: self,
            chat_id,
            sender_chat_id,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's name. Returns True on success."]
    pub fn build_set_my_name<'a>(&'a self) -> CallSetMyName<'a> {
        CallSetMyName {
            bot: self,
            name: None,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to delete a sticker set that was created by the bot. Returns True on success."]
    pub fn build_delete_sticker_set<'a>(&'a self, name: &'a str) -> CallDeleteStickerSet<'a> {
        CallDeleteStickerSet { bot: self, name }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send a native poll. On success, the sent Message is returned."]
    pub fn build_send_poll<'a>(
        &'a self,
        chat_id: i64,
        question: &'a str,
        options: &'a Vec<String>,
    ) -> CallSendPoll<'a> {
        CallSendPoll {
            bot: self,
            chat_id,
            question,
            options,
            message_thread_id: None,
            is_anonymous: None,
            tg_type: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object."]
    pub fn build_get_user_profile_photos<'a>(
        &'a self,
        user_id: i64,
    ) -> CallGetUserProfilePhotos<'a> {
        CallGetUserProfilePhotos {
            bot: self,
            user_id,
            offset: None,
            limit: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns True on success."]
    pub fn build_set_my_short_description<'a>(&'a self) -> CallSetMyShortDescription<'a> {
        CallSetMyShortDescription {
            bot: self,
            short_description: None,
            language_code: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns True on success."]
    pub fn build_create_new_sticker_set<'a>(
        &'a self,
        user_id: i64,
        name: &'a str,
        title: &'a str,
        stickers: &'a Vec<InputSticker>,
        sticker_format: &'a str,
    ) -> CallCreateNewStickerSet<'a> {
        CallCreateNewStickerSet {
            bot: self,
            user_id,
            name,
            title,
            stickers,
            sticker_format,
            sticker_type: None,
            needs_repainting: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned."]
    pub fn build_answer_shipping_query<'a>(
        &'a self,
        shipping_query_id: &'a str,
        ok: bool,
    ) -> CallAnswerShippingQuery<'a> {
        CallAnswerShippingQuery {
            bot: self,
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send text messages. On success, the sent Message is returned."]
    pub fn build_send_message<'a>(&'a self, chat_id: i64, text: &'a str) -> CallSendMessage<'a> {
        CallSendMessage {
            bot: self,
            chat_id,
            text,
            message_thread_id: None,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects."]
    pub fn build_get_updates<'a>(&'a self) -> CallGetUpdates<'a> {
        CallGetUpdates {
            bot: self,
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to send point on the map. On success, the sent Message is returned."]
    pub fn build_send_location<'a>(
        &'a self,
        chat_id: i64,
        latitude: ::ordered_float::OrderedFloat<f64>,
        longitude: ::ordered_float::OrderedFloat<f64>,
    ) -> CallSendLocation<'a> {
        CallSendLocation {
            bot: self,
            chat_id,
            latitude,
            longitude,
            message_thread_id: None,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    #[allow(rustdoc::invalid_html_tags)]
    #[doc = "Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False."]
    pub fn build_set_game_score<'a>(&'a self, user_id: i64, score: i64) -> CallSetGameScore<'a> {
        CallSetGameScore {
            bot: self,
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }
}

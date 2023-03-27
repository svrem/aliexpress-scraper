use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Product {
    pub action_module: ActionModule,
    pub ae_plus_module: AePlusModule,
    pub buyer_protection_module: BuyerProtectionModule,
    pub common_module: CommonModule,
    pub coupon_module: CouponModule,
    pub cross_link_module: CrossLinkModule,
    pub description_module: DescriptionModule,
    pub features: Features8,
    pub feedback_module: FeedbackModule,
    pub group_share_module: GroupShareModule,
    pub i18n_map: I18nMap10,
    pub image_module: ImageModule,
    pub installment_module: InstallmentModule,
    pub middle_banner_module: MiddleBannerModule,
    pub name: String,
    pub other_service_module: OtherServiceModule,
    pub page_module: PageModule,
    pub pre_sale_module: PreSaleModule,
    pub prefix: String,
    pub price_module: PriceModule,
    pub quantity_module: QuantityModule,
    pub recommend_module: RecommendModule,
    pub redirect_module: RedirectModule,
    pub shipping_module: ShippingModule,
    pub sku_module: SkuModule,
    pub specs_module: SpecsModule,
    pub store_module: StoreModule,
    pub title_module: TitleModule,
    pub web_env: WebEnv,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionModule {
    pub action_confs: ActionConfs,
    pub add_to_cart_url: String,
    pub ae_order_from: String,
    pub allow_visitor_add_cart: bool,
    pub cart_detail_url: String,
    pub category_id: i64,
    pub coming_soon: bool,
    pub company_id: i64,
    pub confirm_order_url: String,
    pub features: Features,
    pub freight_ext: String,
    pub i18n_map: I18nMap,
    pub id: i64,
    pub invalid_buy_now: bool,
    pub item_status: i64,
    pub item_wished: bool,
    pub item_wished_count: i64,
    pub local_seller: bool,
    pub name: String,
    pub pre_sale: bool,
    pub product_id: i64,
    pub root_category_id: i64,
    pub show_coin_anim: bool,
    pub show_share_header: bool,
    pub store_num: i64,
    pub switch_info: SwitchInfo,
    pub total_avail_quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionConfs {
    pub buy_now_btn: BuyNowBtn,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyNowBtn {
    pub bg_color_end: String,
    pub bg_color_start: String,
    pub text_color: String,
    pub text_content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap {
    #[serde(rename = "ADDED_TO")]
    pub added_to: String,
    #[serde(rename = "ADD_TO_CART")]
    pub add_to_cart: String,
    #[serde(rename = "ADD_X_MORE")]
    pub add_x_more: String,
    #[serde(rename = "BUYER_ABLE")]
    pub buyer_able: String,
    #[serde(rename = "BUY_NOW")]
    pub buy_now: String,
    #[serde(rename = "COMING_SOON")]
    pub coming_soon: String,
    #[serde(rename = "CONTINUE")]
    pub continue_field: String,
    #[serde(rename = "FIND_SIMILAR")]
    pub find_similar: String,
    #[serde(rename = "NO_LONGER_AVAILABLE")]
    pub no_longer_available: String,
    #[serde(rename = "ORDER_NOW")]
    pub order_now: String,
    #[serde(rename = "PREVIEW_PERIOD")]
    pub preview_period: String,
    #[serde(rename = "SC_ADD_FAILED")]
    pub sc_add_failed: String,
    #[serde(rename = "SC_ADD_MAX")]
    pub sc_add_max: String,
    #[serde(rename = "SC_ADD_SUCC")]
    pub sc_add_succ: String,
    #[serde(rename = "SC_HAVE")]
    pub sc_have: String,
    #[serde(rename = "SC_RECOMMEND")]
    pub sc_recommend: String,
    #[serde(rename = "SC_VIEW")]
    pub sc_view: String,
    #[serde(rename = "SELECT_TIP")]
    pub select_tip: String,
    #[serde(rename = "WISHLIST_PUBLIC_NOTICE")]
    pub wishlist_public_notice: String,
    #[serde(rename = "WISHLIST_PUBLIC_TIP")]
    pub wishlist_public_tip: String,
    #[serde(rename = "WISHLIST_SAVE_BUTTON")]
    pub wishlist_save_button: String,
    #[serde(rename = "WISH_CANCEL_WISHLIST")]
    pub wish_cancel_wishlist: String,
    #[serde(rename = "WISH_CREATE_LIST")]
    pub wish_create_list: String,
    #[serde(rename = "WISH_DETAULT_LIST")]
    pub wish_detault_list: String,
    #[serde(rename = "WISH_MAX_GROUP")]
    pub wish_max_group: String,
    #[serde(rename = "WISH_MAX_NOTICE")]
    pub wish_max_notice: String,
    #[serde(rename = "WISH_MOVE_TO_ANOTHER_LIST_TIPS")]
    pub wish_move_to_another_list_tips: String,
    #[serde(rename = "WISH_NAME_ALREADY_USE")]
    pub wish_name_already_use: String,
    #[serde(rename = "WISH_REVISELIST")]
    pub wish_reviselist: String,
    #[serde(rename = "WISH_SYSTEM_ERROR")]
    pub wish_system_error: String,
    #[serde(rename = "WISH_VIEW_WISH_LIST")]
    pub wish_view_wish_list: String,
    #[serde(rename = "WISH_YOUMAYLIKE")]
    pub wish_youmaylike: String,
    #[serde(rename = "WL_ERROR")]
    pub wl_error: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchInfo {
    pub delivery_migrate: bool,
    pub sku_unavailable_revision: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AePlusModule {
    pub features: Features2,
    pub i18n_map: I18nMap2,
    pub id: i64,
    pub name: String,
    pub tags: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyerProtectionModule {
    pub buyer_protection: BuyerProtection,
    pub features: Features3,
    pub i18n_map: I18nMap3,
    pub id: i64,
    pub name: String,
    pub return_policy_guarantees: ReturnPolicyGuarantees,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyerProtection {
    pub brief: String,
    pub detail_description: String,
    pub id: i64,
    pub name: String,
    pub redirect_url: String,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features3 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap3 {
    #[serde(rename = "PLAZA_BUYER_PROTECTION_CONTENT")]
    pub plaza_buyer_protection_content: String,
    #[serde(rename = "PLAZA_BUYER_PROTECTION_TIP")]
    pub plaza_buyer_protection_tip: String,
    #[serde(rename = "PLAZA_BUYER_PROTECTION_TITLE")]
    pub plaza_buyer_protection_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnPolicyGuarantees {
    pub brief: String,
    pub detail_description: String,
    pub id: i64,
    pub name: String,
    pub redirect_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonModule {
    pub activity: bool,
    pub ae_plus_tags: Vec<Value>,
    pub category_id: i64,
    pub choice_product: bool,
    pub crawler: bool,
    pub currency_code: String,
    pub features: Features4,
    pub gaga_data_site: String,
    pub i18n_map: I18nMap4,
    pub id: i64,
    pub name: String,
    pub plaza: bool,
    pub pre_sale: bool,
    pub product_id: i64,
    pub product_id_str: String,
    pub product_tags: ProductTags,
    pub reminds: Vec<Remind>,
    pub seller_admin_seq: i64,
    pub trade_currency_code: String,
    pub user_country_code: String,
    pub user_country_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features4 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap4 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductTags {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remind {
    pub content: String,
    pub position: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponModule {
    pub currency_code: String,
    pub features: Features5,
    pub i18n_map: I18nMap5,
    pub id: i64,
    pub name: String,
    pub web_coupon_info: WebCouponInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features5 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap5 {
    #[serde(rename = "ADDED")]
    pub added: String,
    #[serde(rename = "BUY_GET_OFF")]
    pub buy_get_off: String,
    #[serde(rename = "CROSS_STORE_VOUCHER")]
    pub cross_store_voucher: String,
    #[serde(rename = "CROSS_STORE_VOUCHER_TIP")]
    pub cross_store_voucher_tip: String,
    #[serde(rename = "EXCHANGE_MORE")]
    pub exchange_more: String,
    #[serde(rename = "EXCHANGE_NOW")]
    pub exchange_now: String,
    #[serde(rename = "GET")]
    pub get: String,
    #[serde(rename = "GET_COUPONS")]
    pub get_coupons: String,
    #[serde(rename = "GET_IT")]
    pub get_it: String,
    #[serde(rename = "GET_MORE")]
    pub get_more: String,
    #[serde(rename = "GET_NOW")]
    pub get_now: String,
    #[serde(rename = "INSTANT_DISCOUNT")]
    pub instant_discount: String,
    #[serde(rename = "MY_BALANCE")]
    pub my_balance: String,
    #[serde(rename = "MY_COINS")]
    pub my_coins: String,
    #[serde(rename = "NEW_USER_COUPON")]
    pub new_user_coupon: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL")]
    pub new_user_coupon_acquire_fail: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_ALREADY_HAVE")]
    pub new_user_coupon_acquire_fail_already_have: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_GRANT_ERROR")]
    pub new_user_coupon_acquire_fail_grant_error: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_GROUP_LIMIT")]
    pub new_user_coupon_acquire_fail_group_limit: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_LIMIT_DAY")]
    pub new_user_coupon_acquire_fail_limit_day: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_LIMIT_HOUR")]
    pub new_user_coupon_acquire_fail_limit_hour: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_NOT_AVAILABLE_NOW")]
    pub new_user_coupon_acquire_fail_not_available_now: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_NOT_NEW_USER")]
    pub new_user_coupon_acquire_fail_not_new_user: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_REGISTER_COUNTRY_LIMIT")]
    pub new_user_coupon_acquire_fail_register_country_limit: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_SECURITY")]
    pub new_user_coupon_acquire_fail_security: String,
    #[serde(rename = "NEW_USER_COUPON_ACQUIRE_FAIL_SYSTEM_ERROR")]
    pub new_user_coupon_acquire_fail_system_error: String,
    #[serde(rename = "OFF_ON")]
    pub off_on: String,
    #[serde(rename = "OFF_PER")]
    pub off_per: String,
    #[serde(rename = "ORDER_OVER")]
    pub order_over: String,
    #[serde(rename = "PANEL_TITLE")]
    pub panel_title: String,
    #[serde(rename = "SCP_ERR_HAVE")]
    pub scp_err_have: String,
    #[serde(rename = "SCP_ERR_NONE")]
    pub scp_err_none: String,
    #[serde(rename = "SHOPPONG_CREDIT")]
    pub shoppong_credit: String,
    #[serde(rename = "SHOP_COUPONE_TIME_EXPIRES")]
    pub shop_coupone_time_expires: String,
    #[serde(rename = "SHOP_COUPONE_TIME_START_END")]
    pub shop_coupone_time_start_end: String,
    #[serde(rename = "SHOP_PROMO_CODE_COPIED")]
    pub shop_promo_code_copied: String,
    #[serde(rename = "SHOP_PROMO_CODE_COP_FAILED")]
    pub shop_promo_code_cop_failed: String,
    #[serde(rename = "SHOP_PROMO_CODE_TITLE")]
    pub shop_promo_code_title: String,
    #[serde(rename = "STORE_COUPON")]
    pub store_coupon: String,
    #[serde(rename = "code-30005Title")]
    pub code_30005title: String,
    #[serde(rename = "code14Title")]
    pub code14title: String,
    #[serde(rename = "code17Title")]
    pub code17title: String,
    #[serde(rename = "code50Title")]
    pub code50title: String,
    #[serde(rename = "code51Title")]
    pub code51title: String,
    #[serde(rename = "code52Title")]
    pub code52title: String,
    pub system_error: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCouponInfo {
    pub coupon_attributes: CouponAttributes,
    pub coupon_list: Vec<CouponList>,
    #[serde(rename = "promotionPanelDTO")]
    pub promotion_panel_dto: PromotionPanelDto,
    pub using_new_coupon_api: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponAttributes {
    pub coupon_package_text: String,
    pub has_platform_shipping_fee_coupon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponList {
    pub bg_color: String,
    pub color: String,
    pub copy: String,
    pub order: i64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionPanelDto {
    pub shop_coupon: Vec<ShopCoupon>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopCoupon {
    pub display_type: String,
    pub promotion_group_title: String,
    #[serde(rename = "promotionPanelDetailDTOList")]
    pub promotion_panel_detail_dtolist: Vec<PromotionPanelDetailDtolist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionPanelDetailDtolist {
    pub acquirable: bool,
    pub action_tips: String,
    pub attributes: Attributes,
    pub coupon_assign_param: CouponAssignParam,
    pub display_type: String,
    pub end_time: i64,
    pub promotion_desc: String,
    pub promotion_detail: String,
    pub start_time: i64,
    pub teasing_start_time: i64,
    pub tool_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub activity_id: String,
    pub code_scope: String,
    pub collected_tips: String,
    pub coupon_campaign_style: String,
    pub coupon_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponAssignParam {
    pub coupon_meta: String,
    pub coupon_type: String,
    pub promotion_id: i64,
    pub seller_admin_id: i64,
    pub seller_coupon_activity_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossLinkModule {
    pub bread_crumb_path_list: Vec<BreadCrumbPathList>,
    pub cross_link_group_list: Vec<CrossLinkGroupList>,
    pub features: Features6,
    pub i18n_map: I18nMap6,
    pub id: i64,
    pub name: String,
    pub product_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreadCrumbPathList {
    pub cate_id: i64,
    pub name: String,
    pub remark: String,
    pub url: String,
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossLinkGroupList {
    pub channel: String,
    pub cross_link_list: Vec<CrossLinkList>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossLinkList {
    pub display_name: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features6 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap6 {
    #[serde(rename = "BREADCRUMB_PART1")]
    pub breadcrumb_part1: String,
    #[serde(rename = "BREADCRUMB_PART2")]
    pub breadcrumb_part2: String,
    #[serde(rename = "CROSSLINK_RELATED_SEARCHES")]
    pub crosslink_related_searches: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionModule {
    pub description_url: String,
    pub features: Features7,
    pub i18n_map: I18nMap7,
    pub id: i64,
    pub name: String,
    pub product_id: i64,
    pub seller_admin_seq: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features7 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap7 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features8 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackModule {
    pub company_id: i64,
    pub features: Features9,
    pub feedback_server: String,
    pub i18n_map: I18nMap8,
    pub id: i64,
    pub name: String,
    pub product_id: i64,
    pub seller_admin_seq: i64,
    pub trial_review_num: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features9 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap8 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupShareModule {
    pub features: Features10,
    pub i18n_map: I18nMap9,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features10 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap9 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap10 {
    #[serde(rename = "ASK_BUYERS")]
    pub ask_buyers: String,
    #[serde(rename = "PAGE_NOT_FOUND_NOTICE")]
    pub page_not_found_notice: String,
    #[serde(rename = "PAGE_NOT_FOUND_RCMD_TITLE")]
    pub page_not_found_rcmd_title: String,
    #[serde(rename = "VIEW_5_MORE_ANSWERS")]
    pub view_5_more_answers: String,
    #[serde(rename = "VIEW_MORE")]
    pub view_more: String,
    pub open_trace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageModule {
    pub approved: bool,
    pub features: Features11,
    pub i18n_map: I18nMap11,
    pub id: i64,
    pub image_path_list: Vec<String>,
    pub name: String,
    pub summ_image_path_list: Vec<String>,
    pub video_id: i64,
    pub video_uid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features11 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap11 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallmentModule {
    pub features: Features12,
    pub i18n_map: I18nMap12,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features12 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap12 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiddleBannerModule {
    pub features: Features13,
    pub i18n_map: I18nMap13,
    pub id: i64,
    pub name: String,
    pub show_uniform_banner: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features13 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap13 {
    #[serde(rename = "DAY")]
    pub day: String,
    #[serde(rename = "DAYS")]
    pub days: String,
    #[serde(rename = "END_IN")]
    pub end_in: String,
    #[serde(rename = "STARTS_IN")]
    pub starts_in: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherServiceModule {
    pub features: Features14,
    pub has_warranty_info: bool,
    pub i18n_map: I18nMap14,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features14 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap14 {
    #[serde(rename = "PLAZA_SERVICE_CONTENT1_PC")]
    pub plaza_service_content1_pc: String,
    #[serde(rename = "PLAZA_SERVICE_CONTENT2_PC")]
    pub plaza_service_content2_pc: String,
    #[serde(rename = "PLAZA_SERVICE_CONTENT3_1_PC")]
    pub plaza_service_content3_1_pc: String,
    #[serde(rename = "PLAZA_SERVICE_CONTENT3_2_PC")]
    pub plaza_service_content3_2_pc: String,
    #[serde(rename = "PLAZA_SERVICE_CONTENT3_3_PC")]
    pub plaza_service_content3_3_pc: String,
    #[serde(rename = "PLAZA_SERVICE_SUBTITLE2_PC")]
    pub plaza_service_subtitle2_pc: String,
    #[serde(rename = "PLAZA_SERVICE_SUBTITLE3_PC")]
    pub plaza_service_subtitle3_pc: String,
    #[serde(rename = "PLAZA_SERVICE_SUBTITLE_PC")]
    pub plaza_service_subtitle_pc: String,
    #[serde(rename = "PLAZA_SERVICE_TITLE_PC")]
    pub plaza_service_title_pc: String,
    #[serde(rename = "PLAZA_SERVICE_WARRANTY_BRAND")]
    pub plaza_service_warranty_brand: String,
    #[serde(rename = "PLAZA_SERVICE_WARRANTY_CATEGORY")]
    pub plaza_service_warranty_category: String,
    #[serde(rename = "PLAZA_SERVICE_WARRANTY_EMAIL")]
    pub plaza_service_warranty_email: String,
    #[serde(rename = "PLAZA_SERVICE_WARRANTY_HOURS")]
    pub plaza_service_warranty_hours: String,
    #[serde(rename = "PLAZA_SERVICE_WARRANTY_PHONE")]
    pub plaza_service_warranty_phone: String,
    #[serde(rename = "PLAZA_SERVICE_WARRANTY_TITLE")]
    pub plaza_service_warranty_title: String,
    #[serde(rename = "PLAZA_SERVICE_WARRANTY_WEBSITE")]
    pub plaza_service_warranty_website: String,
    #[serde(rename = "TAB_CUSTOMER_REVIEWS")]
    pub tab_customer_reviews: String,
    #[serde(rename = "TAB_OVERVIEW")]
    pub tab_overview: String,
    #[serde(rename = "TAB_REPORT_ITEM")]
    pub tab_report_item: String,
    #[serde(rename = "TAB_SERVICE")]
    pub tab_service: String,
    #[serde(rename = "TAB_SPECS")]
    pub tab_specs: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModule {
    pub ae_order_from: String,
    pub aer_self_operation: bool,
    pub amphtml_tag: String,
    pub boutique_seller: bool,
    pub canonical_tag: String,
    pub complaint_url: String,
    pub description: String,
    pub features: Features15,
    pub hreflang_tag: String,
    pub i18n_map: I18nMap15,
    pub id: i64,
    pub image_path: String,
    pub item_detail_url: String,
    pub keywords: String,
    pub kid_baby: bool,
    pub m_site_url: String,
    pub media_tag: String,
    pub multi_language_url_list: Vec<MultiLanguageUrlList>,
    pub name: String,
    pub og_description: String,
    pub og_title: String,
    pub ogurl: String,
    pub old_item_detail_url: String,
    pub plaza_electronic_seller: bool,
    pub product_id: i64,
    pub ru_self_operation: bool,
    pub show_plaza_header: bool,
    pub site_type: String,
    pub spanish_plaza: bool,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features15 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap15 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiLanguageUrlList {
    pub language: String,
    pub language_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreSaleModule {
    pub features: Features16,
    pub i18n_map: I18nMap16,
    pub id: i64,
    pub name: String,
    pub pre_sale: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features16 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap16 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceModule {
    pub activity: bool,
    pub discount: i64,
    pub discount_promotion: bool,
    pub discount_ratio_tips: String,
    pub discount_tips: String,
    pub extra_tags: ExtraTags,
    pub features: Features17,
    pub formated_activity_price: String,
    pub formated_price: String,
    pub hidden_big_sale_price: bool,
    pub i18n_map: I18nMap17,
    pub id: i64,
    pub installment: bool,
    pub lot: bool,
    pub max_activity_amount: MaxActivityAmount,
    pub max_amount: MaxAmount,
    pub min_activity_amount: MinActivityAmount,
    pub min_amount: MinAmount,
    pub multi_unit_name: String,
    pub name: String,
    pub number_per_lot: i64,
    pub odd_unit_name: String,
    pub pre_sale: bool,
    pub price_rule_info: PriceRuleInfo,
    pub promotion_selling_point_tags: Vec<Value>,
    pub regular_price_activity: bool,
    pub show_activity_message: bool,
    pub vat_desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraTags {
    pub separator_icon: SeparatorIcon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeparatorIcon {
    pub element_list: Vec<ElementList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementList {
    pub element_type: String,
    pub icon_address: String,
    pub icon_height: i64,
    pub icon_type: String,
    pub icon_width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features17 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap17 {
    #[serde(rename = "DEPOSIT")]
    pub deposit: String,
    #[serde(rename = "INSTALLMENT")]
    pub installment: String,
    #[serde(rename = "LOT")]
    pub lot: String,
    #[serde(rename = "PRE_ORDER_PRICE")]
    pub pre_order_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxActivityAmount {
    pub currency: String,
    pub formated_amount: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxAmount {
    pub currency: String,
    pub formated_amount: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinActivityAmount {
    pub currency: String,
    pub formated_amount: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinAmount {
    pub currency: String,
    pub formated_amount: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceRuleInfo {
    pub hint_copywriting: String,
    pub layer_info: LayerInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInfo {
    pub main_intro: String,
    pub price_copywriting: String,
    pub show_price_copywriting: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityModule {
    pub activity: bool,
    pub display_bulk_info: bool,
    pub features: Features18,
    pub i18n_map: I18nMap18,
    pub id: i64,
    pub lot: bool,
    pub multi_unit_name: String,
    pub name: String,
    pub odd_unit_name: String,
    pub purchase_limit_num_max: i64,
    pub sku_bulk_discount: i64,
    pub sku_bulk_order: i64,
    pub total_avail_quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features18 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap18 {
    #[serde(rename = "ADDTIONAL")]
    pub addtional: String,
    #[serde(rename = "BUY_LIMIT")]
    pub buy_limit: String,
    #[serde(rename = "LOT")]
    pub lot: String,
    #[serde(rename = "LOTS")]
    pub lots: String,
    #[serde(rename = "OFF_OR_MORE")]
    pub off_or_more: String,
    #[serde(rename = "ONLY_QUANTITY_LEFT")]
    pub only_quantity_left: String,
    #[serde(rename = "QUANTITY")]
    pub quantity: String,
    #[serde(rename = "QUANTITY_AVAILABLE")]
    pub quantity_available: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendModule {
    pub category_id: i64,
    pub company_id: i64,
    pub features: Features19,
    pub i18n_map: I18nMap19,
    pub id: i64,
    pub name: String,
    pub platform_count: i64,
    pub product_id: i64,
    pub store_num: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features19 {
    pub recommend_gps_scenario_other_seller_products: String,
    pub recommend_gps_scenario_seller_other_products: String,
    pub recommend_gps_scenario_top_selling: String,
    pub show_sub_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap19 {
    #[serde(rename = "FROM_OTHER_SELLER")]
    pub from_other_seller: String,
    #[serde(rename = "MORE_FROM_THIS_SELLER")]
    pub more_from_this_seller: String,
    #[serde(rename = "PRODUCT_SOLD")]
    pub product_sold: String,
    #[serde(rename = "TOP_SELLING")]
    pub top_selling: String,
    #[serde(rename = "VIEW_MORE_LINK")]
    pub view_more_link: String,
    #[serde(rename = "YOU_MAY_LIKE")]
    pub you_may_like: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedirectModule {
    pub big_boss_ban: bool,
    pub code: String,
    pub features: Features20,
    pub i18n_map: I18nMap20,
    pub id: i64,
    pub name: String,
    pub redirect_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features20 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap20 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingModule {
    pub currency_code: String,
    pub features: Features21,
    pub freight_calculate_info: FreightCalculateInfo,
    pub general_freight_info: GeneralFreightInfo,
    pub hba_free_shipping: bool,
    pub hba_freight: bool,
    pub i18n_map: I18nMap21,
    pub id: i64,
    pub name: String,
    pub product_id: i64,
    pub region_country_name: String,
    pub suppress_freight_invoke: bool,
    pub user_country_code: String,
    pub user_country_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features21 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreightCalculateInfo {
    pub display_multiple_freight: bool,
    pub hide_freight: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralFreightInfo {
    pub allow_arouse_layer: bool,
    pub hide_ship_from: bool,
    pub original_layout_result_list: Vec<OriginalLayoutResultList>,
    pub shipping_fee_text: String,
    pub using_new_freight: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalLayoutResultList {
    pub addition_layout: Vec<AdditionLayout>,
    pub biz_data: BizData,
    pub content_layout: Vec<Vec<ContentLayout>>,
    pub delivery_option_panel_display_list: Vec<String>,
    pub detail_first_screen_display_list: Vec<String>,
    pub dx_template_info: DxTemplateInfo,
    pub final_pattern_id: i64,
    pub sku_switch_display_list: Vec<String>,
    pub title_layout: Vec<Vec<TitleLayout>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionLayout {
    pub block_group: String,
    pub block_type: String,
    pub component_id: String,
    pub content: String,
    pub display_condition: String,
    pub distance: i64,
    pub medusa_key: String,
    pub order: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub use_in_delivery_option_panel: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BizData {
    pub company: String,
    pub currency: String,
    pub delivery_date: String,
    pub delivery_day_max: i64,
    pub delivery_day_min: i64,
    pub delivery_option_code: String,
    pub delivery_provider_code: String,
    pub delivery_provider_name: String,
    pub discount: f64,
    pub eta_trace_id: String,
    pub freight_commit_day: String,
    pub guaranteed_delivery_time: i64,
    pub guaranteed_delivery_time_provider_name: String,
    pub item_id: i64,
    pub item_scene: String,
    pub item_ship_from_type: String,
    pub lead_time_sort: Vec<i64>,
    pub pattern_id: i64,
    pub provider: String,
    pub ship_from: String,
    pub ship_from_code: String,
    pub ship_to: String,
    pub ship_to_code: String,
    pub shipping_fee: String,
    #[serde(rename = "showXDayDeliveryTips")]
    pub show_xday_delivery_tips: String,
    pub solution_business_type: String,
    pub solution_id: i64,
    pub tracking: String,
    pub ut_params: String,
    pub display_amount: Option<f64>,
    pub display_currency: Option<String>,
    pub formatted_amount: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentLayout {
    pub block_group: String,
    pub block_type: String,
    pub component_id: String,
    pub content: String,
    pub display_condition: String,
    pub distance: i64,
    pub medusa_key: String,
    pub order: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub use_in_delivery_option_panel: bool,
    pub use_in_detail_first_screen: bool,
    pub use_in_sku_switch: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxTemplateInfo {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleLayout {
    pub block_group: String,
    pub block_type: String,
    pub component_id: String,
    pub content: String,
    pub display_condition: String,
    pub distance: i64,
    pub medusa_key: String,
    pub order: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub use_in_delivery_option_panel: bool,
    pub use_in_detail_first_screen: bool,
    pub use_in_sku_switch: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap21 {
    #[serde(rename = "APPLY")]
    pub apply: String,
    #[serde(rename = "BALLOON_TIP")]
    pub balloon_tip: String,
    #[serde(rename = "CAN_NOE_DELIVER_NOTE")]
    pub can_noe_deliver_note: String,
    #[serde(rename = "CAN_NOT_DELIVER")]
    pub can_not_deliver: String,
    #[serde(rename = "CARRIER")]
    pub carrier: String,
    #[serde(rename = "CHOOSE_DELIVERT_METHOD")]
    pub choose_delivert_method: String,
    #[serde(rename = "COST")]
    pub cost: String,
    #[serde(rename = "DAYS")]
    pub days: String,
    #[serde(rename = "DELIVERED_BY")]
    pub delivered_by: String,
    #[serde(rename = "ESTIMATED_DELIVERT_ON_DATE")]
    pub estimated_delivert_on_date: String,
    #[serde(rename = "ESTIMATED_DELIVERT_ON_DAYS")]
    pub estimated_delivert_on_days: String,
    #[serde(rename = "ESTIMATED_DELIVERY")]
    pub estimated_delivery: String,
    #[serde(rename = "FAST_SHIPPING")]
    pub fast_shipping: String,
    #[serde(rename = "FREE_SHIPPING")]
    pub free_shipping: String,
    #[serde(rename = "GENERAL_SHIPPIING_FROM")]
    pub general_shippiing_from: String,
    #[serde(rename = "GENERAL_SHIPPING_DELIVERY")]
    pub general_shipping_delivery: String,
    #[serde(rename = "GENERAL_SHIPPING_TO")]
    pub general_shipping_to: String,
    #[serde(rename = "GENERAL_SHPPING_MORE")]
    pub general_shpping_more: String,
    #[serde(rename = "HAB_BALLOON_DOOR_DELIVERY")]
    pub hab_balloon_door_delivery: String,
    #[serde(rename = "HAB_BALLOON_TRAKING_AVAILABLE")]
    pub hab_balloon_traking_available: String,
    #[serde(rename = "HAB_BALLOON_VAT_INCLUDED")]
    pub hab_balloon_vat_included: String,
    #[serde(rename = "HAB_SHIPPING_TO")]
    pub hab_shipping_to: String,
    #[serde(rename = "HBA_BALLOON_TIPS")]
    pub hba_balloon_tips: String,
    #[serde(rename = "HBA_DOR_DELIVERY")]
    pub hba_dor_delivery: String,
    #[serde(rename = "HBA_SHIPPING_INFO")]
    pub hba_shipping_info: String,
    #[serde(rename = "HBA_TRAKING_AVAILABLE")]
    pub hba_traking_available: String,
    #[serde(rename = "HBA_TVAT_INCLUDED")]
    pub hba_tvat_included: String,
    #[serde(rename = "IN")]
    pub in_field: String,
    #[serde(rename = "LOGISTIC_COMPOSE_AE")]
    pub logistic_compose_ae: String,
    #[serde(rename = "LOGISTIC_COMPOSE_BRAND_MIND")]
    pub logistic_compose_brand_mind: String,
    #[serde(rename = "LOGISTIC_COMPOSE_ORDERS_OVER")]
    pub logistic_compose_orders_over: String,
    #[serde(rename = "LOGISTIC_COMPOSE_SPEED_UP")]
    pub logistic_compose_speed_up: String,
    #[serde(rename = "OR_FULL_REFOUND")]
    pub or_full_refound: String,
    #[serde(rename = "PLAZA_BALLOON_TIP")]
    pub plaza_balloon_tip: String,
    #[serde(rename = "PLAZA_SHOP_NOW_RECEIVE_ON")]
    pub plaza_shop_now_receive_on: String,
    #[serde(rename = "SEARCH")]
    pub search: String,
    #[serde(rename = "SELECTED")]
    pub selected: String,
    #[serde(rename = "SELECT_SHIP_FROM")]
    pub select_ship_from: String,
    #[serde(rename = "SELECT_SHIP_FROM_TIP")]
    pub select_ship_from_tip: String,
    #[serde(rename = "SHIPPING_TO")]
    pub shipping_to: String,
    #[serde(rename = "SHIP_MY_ITEM_TO")]
    pub ship_my_item_to: String,
    #[serde(rename = "TO_CITY")]
    pub to_city: String,
    #[serde(rename = "TO_COUNTRY")]
    pub to_country: String,
    #[serde(rename = "TO_PROVINCE")]
    pub to_province: String,
    #[serde(rename = "TO_VIA")]
    pub to_via: String,
    #[serde(rename = "TO_WHERE")]
    pub to_where: String,
    #[serde(rename = "TRACKING")]
    pub tracking: String,
    #[serde(rename = "VAT_DE_DETAIL")]
    pub vat_de_detail: String,
    #[serde(rename = "VAT_NUMBER")]
    pub vat_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuModule {
    pub category_id: i64,
    pub features: Features22,
    pub force_promise_warranty_json: String,
    pub has_size_info: bool,
    pub has_sku_property: bool,
    pub i18n_map: I18nMap22,
    pub id: i64,
    pub name: String,
    #[serde(rename = "productSKUPropertyList")]
    pub product_skuproperty_list: Vec<ProductSkupropertyList>,
    pub selected_sku_attr: String,
    pub sku_price_list: Vec<SkuPriceList>,
    pub warranty_detail_json: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features22 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap22 {
    #[serde(rename = "AXIS")]
    pub axis: String,
    #[serde(rename = "BTN_CANCEL")]
    pub btn_cancel: String,
    #[serde(rename = "BTN_SAVE")]
    pub btn_save: String,
    #[serde(rename = "BUST_CONTENT")]
    pub bust_content: String,
    #[serde(rename = "BUST_PROMPT")]
    pub bust_prompt: String,
    #[serde(rename = "BUST_TITLE")]
    pub bust_title: String,
    #[serde(rename = "CUSTOM_SIZE_CONTENT")]
    pub custom_size_content: String,
    #[serde(rename = "CYL")]
    pub cyl: String,
    #[serde(rename = "FLOOR_CONTENT")]
    pub floor_content: String,
    #[serde(rename = "FLOOR_PROMPT")]
    pub floor_prompt: String,
    #[serde(rename = "FLOOR_TITLE")]
    pub floor_title: String,
    #[serde(rename = "GLASSES_DIALOG_TITLE")]
    pub glasses_dialog_title: String,
    #[serde(rename = "GLASSES_TIP")]
    pub glasses_tip: String,
    #[serde(rename = "HEIGHT_PROMPT")]
    pub height_prompt: String,
    #[serde(rename = "HIPS_CONTENT")]
    pub hips_content: String,
    #[serde(rename = "HIPS_PROMPT")]
    pub hips_prompt: String,
    #[serde(rename = "HIPS_TITLE")]
    pub hips_title: String,
    #[serde(rename = "HOW_TO_MEASURE")]
    pub how_to_measure: String,
    #[serde(rename = "ITEM_CONDITION_TIP")]
    pub item_condition_tip: String,
    #[serde(rename = "NV_ADD")]
    pub nv_add: String,
    #[serde(rename = "NV_ADD_PROMPT")]
    pub nv_add_prompt: String,
    #[serde(rename = "PLEASE_ENTER")]
    pub please_enter: String,
    #[serde(rename = "PUPILLARY_PROMPT")]
    pub pupillary_prompt: String,
    #[serde(rename = "SELECT")]
    pub select: String,
    #[serde(rename = "SERVICE")]
    pub service: String,
    #[serde(rename = "SIZE_CHART")]
    pub size_chart: String,
    #[serde(rename = "SIZE_DIALOG_TITLE")]
    pub size_dialog_title: String,
    #[serde(rename = "SIZE_HOVER_TITLE")]
    pub size_hover_title: String,
    #[serde(rename = "SIZE_INFO")]
    pub size_info: String,
    #[serde(rename = "SIZE_INFO_COMPARE_TIP")]
    pub size_info_compare_tip: String,
    #[serde(rename = "SIZE_INFO_DESC")]
    pub size_info_desc: String,
    #[serde(rename = "SIZE_INFO_TIP")]
    pub size_info_tip: String,
    #[serde(rename = "SIZING_INFO")]
    pub sizing_info: String,
    #[serde(rename = "SPH")]
    pub sph: String,
    #[serde(rename = "SPH_PROMPT")]
    pub sph_prompt: String,
    #[serde(rename = "TITLE_OPTIONAL")]
    pub title_optional: String,
    #[serde(rename = "UNIT_PROMPT")]
    pub unit_prompt: String,
    #[serde(rename = "WAIST_CONTENT")]
    pub waist_content: String,
    #[serde(rename = "WAIST_PROMPT")]
    pub waist_prompt: String,
    #[serde(rename = "WAIST_TITLE")]
    pub waist_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSkupropertyList {
    pub is_show_type_color: bool,
    pub order: i64,
    pub show_type: String,
    pub show_type_color: bool,
    pub sku_property_id: i64,
    pub sku_property_name: String,
    pub sku_property_values: Vec<SkuPropertyValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuPropertyValue {
    pub property_value_definition_name: String,
    pub property_value_display_name: String,
    pub property_value_id: i64,
    pub property_value_id_long: i64,
    pub property_value_name: String,
    pub sku_color_value: String,
    pub sku_property_image_path: String,
    pub sku_property_image_summ_path: String,
    pub sku_property_tips: String,
    pub sku_property_value_show_order: i64,
    pub sku_property_value_tips: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuPriceList {
    pub freight_ext: String,
    pub salable: bool,
    pub sku_attr: String,
    pub sku_id: i64,
    pub sku_id_str: String,
    pub sku_prop_ids: String,
    pub sku_val: SkuVal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuVal {
    pub avail_quantity: i64,
    pub bulk_order: i64,
    pub discount: String,
    pub discount_tips: String,
    pub inventory: i64,
    pub is_activity: bool,
    pub optional_warranty_price: Vec<Value>,
    pub sku_activity_amount: SkuActivityAmount,
    pub sku_amount: SkuAmount,
    pub sku_cal_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuActivityAmount {
    pub currency: String,
    pub formated_amount: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuAmount {
    pub currency: String,
    pub formated_amount: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecsModule {
    pub features: Features23,
    pub i18n_map: I18nMap23,
    pub id: i64,
    pub name: String,
    pub props: Vec<Prop>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features23 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap23 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prop {
    pub attr_name: String,
    pub attr_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreModule {
    pub company_id: i64,
    pub country_complete_name: String,
    pub detail_page_url: String,
    pub es_retail_or_consignment: bool,
    pub features: Features24,
    pub feedback_message_server: String,
    pub feedback_server: String,
    pub followed: bool,
    pub following_number: i64,
    pub has_store: bool,
    pub has_store_header: bool,
    pub hide_customer_service: bool,
    pub i18n_map: I18nMap24,
    pub id: i64,
    pub name: String,
    pub open_time: String,
    pub opened_year: i64,
    pub positive_num: i64,
    pub positive_rate: String,
    pub product_id: i64,
    pub seller_admin_seq: i64,
    pub session_id: String,
    pub site_type: String,
    pub store_name: String,
    pub store_num: i64,
    #[serde(rename = "storeURL")]
    pub store_url: String,
    #[serde(rename = "topBrandDescURL")]
    pub top_brand_desc_url: String,
    pub top_rated_seller: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features24 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap24 {
    #[serde(rename = "CONTACT_SELLER")]
    pub contact_seller: String,
    #[serde(rename = "COUSTOMER_SERVICE")]
    pub coustomer_service: String,
    #[serde(rename = "FOLLOWER")]
    pub follower: String,
    #[serde(rename = "FOLLOWERS")]
    pub followers: String,
    #[serde(rename = "FOLLOWING_STATE")]
    pub following_state: String,
    #[serde(rename = "POSITIVE_FEEDBACK")]
    pub positive_feedback: String,
    #[serde(rename = "STORE_CATEGORIES")]
    pub store_categories: String,
    #[serde(rename = "TOP_SELLER")]
    pub top_seller: String,
    #[serde(rename = "UNFOLLOWING_STATE")]
    pub unfollowing_state: String,
    #[serde(rename = "VISIT_STORE")]
    pub visit_store: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleModule {
    pub features: Features25,
    pub feedback_rating: FeedbackRating,
    pub format_trade_count: String,
    pub i18n_map: I18nMap25,
    pub id: i64,
    pub name: String,
    pub orig: bool,
    pub orig_title: bool,
    pub subject: String,
    pub title_tags: Vec<Value>,
    pub trade_count: i64,
    pub trade_count_unit: String,
    pub trans: bool,
    pub trans_title: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features25 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackRating {
    pub average_star: String,
    pub average_star_rage: String,
    pub display: bool,
    pub evarage_star: String,
    pub evarage_star_rage: String,
    pub five_star_num: i64,
    pub five_star_rate: String,
    pub four_star_num: i64,
    pub four_star_rate: String,
    pub one_star_num: i64,
    pub one_star_rate: String,
    pub positive_rate: String,
    pub three_star_num: i64,
    pub three_star_rate: String,
    pub total_valid_num: i64,
    pub trial_review_num: i64,
    pub two_star_num: i64,
    pub two_star_rate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nMap25 {
    #[serde(rename = "FREEBIE_REVIEW")]
    pub freebie_review: String,
    #[serde(rename = "FREEBIE_REVIEWS")]
    pub freebie_reviews: String,
    #[serde(rename = "REVIEW")]
    pub review: String,
    #[serde(rename = "REVIEWS")]
    pub reviews: String,
    #[serde(rename = "VIEW_ALL_REVIEWS")]
    pub view_all_reviews: String,
    #[serde(rename = "VIEW_EN_TITLE")]
    pub view_en_title: String,
    #[serde(rename = "VIEW_OTHER_TITLE")]
    pub view_other_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebEnv {
    pub country: String,
    pub currency: String,
    pub env: Env,
    pub host: String,
    pub hostname: String,
    pub ip: String,
    pub lang: String,
    pub language: String,
    pub locale: String,
    pub referer: String,
    pub req_host: String,
    pub site: String,
    pub trace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub val_map: ValMap,
    pub zone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValMap {
    #[serde(rename = "g11n:country")]
    pub g11n_country: String,
    #[serde(rename = "g11n:currency")]
    pub g11n_currency: String,
    #[serde(rename = "g11n:locale")]
    pub g11n_locale: String,
    #[serde(rename = "g11n:site")]
    pub g11n_site: String,
    #[serde(rename = "g11n:timezone")]
    pub g11n_timezone: String,
    #[serde(rename = "page:app")]
    pub page_app: String,
    #[serde(rename = "page:id")]
    pub page_id: String,
    #[serde(rename = "page:name")]
    pub page_name: String,
    #[serde(rename = "ua:browser")]
    pub ua_browser: String,
    #[serde(rename = "ua:device")]
    pub ua_device: String,
    #[serde(rename = "ua:platform")]
    pub ua_platform: String,
    #[serde(rename = "user:id")]
    pub user_id: String,
    #[serde(rename = "user:member")]
    pub user_member: String,
    #[serde(rename = "user:type")]
    pub user_type: String,
}

# Postmark endpoint coverage

Scope: endpoints represented in this crate.

Legend:

- `x` implemented
- `-` not implemented
- `deprecated` intentionally excluded

## Email

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| POST | `/email` | server | x | `api::email::SendEmailRequest` |
| POST | `/email/batch` | server | x | `api::email::SendEmailBatchRequest` |
| POST | `/email/withTemplate` | server | x | `api::email::SendEmailWithTemplateRequest` |
| POST | `/email/batchWithTemplates` | server | x | `api::email::SendEmailBatchWithTemplatesRequest` |

## Bulk

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| POST | `/email/bulk` | server | x | `api::bulk::SendBulkEmailRequest` |
| GET | `/email/bulk/{id}` | server | x | `api::bulk::GetBulkStatusRequest` |

## Bounce

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/deliverystats` | server | x | `api::bounce::GetDeliveryStatsRequest` |
| GET | `/bounces` | server | x | `api::bounce::ListBouncesRequest` |
| GET | `/bounces/{id}` | server | x | `api::bounce::GetBounceRequest` |
| GET | `/bounces/{id}/dump` | server | x | `api::bounce::GetBounceDumpRequest` |
| PUT | `/bounces/{id}/activate` | server | x | `api::bounce::ActivateBounceRequest` |

## Templates

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/templates/{id}` | server | x | `api::templates::GetTemplateRequest` |
| POST | `/templates` | server | x | `api::templates::CreateTemplateRequest` |
| PUT | `/templates/{id}` | server | x | `api::templates::EditTemplateRequest` |
| GET | `/templates?count={count}&offset={offset}` | server | x | `api::templates::ListTemplatesRequest` |
| DELETE | `/templates/{id}` | server | x | `api::templates::DeleteTemplateRequest` |
| POST | `/templates/validate` | server | x | `api::templates::ValidateTemplateRequest` |
| PUT | `/templates/push` | server | x | `api::templates::PushTemplatesRequest` |

## Server / Servers

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/server` | server | x | `api::server::GetCurrentServerRequest` |
| PUT | `/server` | server | x | `api::server::EditServerRequest` |
| GET | `/servers/{id}` | account | x | `api::server::GetServerRequest` |
| POST | `/servers` | account | x | `api::server::CreateServerRequest` |
| PUT | `/servers/{id}` | account | x | `api::server::EditServerByIdRequest` |
| GET | `/servers?count={count}&offset={offset}` | account | x | `api::server::ListServersRequest` |
| DELETE | `/servers/{id}` | account | x | `api::server::DeleteServerRequest` |

## Message Streams / Suppressions

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/message-streams` | server | x | `api::message_streams::ListMessageStreamsRequest` |
| GET | `/message-streams/{id}` | server | x | `api::message_streams::GetMessageStreamRequest` |
| PATCH | `/message-streams/{id}` | server | x | `api::message_streams::EditMessageStreamRequest` |
| POST | `/message-streams` | server | x | `api::message_streams::CreateMessageStreamRequest` |
| POST | `/message-streams/{id}/archive` | server | x | `api::message_streams::ArchiveMessageStreamRequest` |
| POST | `/message-streams/{id}/unarchive` | server | x | `api::message_streams::UnarchiveMessageStreamRequest` |
| GET | `/message-streams/{id}/suppressions/dump` | server | x | `api::message_streams::GetSuppressionsRequest` |
| POST | `/message-streams/{id}/suppressions` | server | x | `api::message_streams::CreateSuppressionRequest` |
| PUT | `/message-streams/{id}/suppressions/{email}` | server | x | `api::message_streams::DeleteSuppressionRequest` |

## Domains

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/domains` | account | x | `api::domains::ListDomainsRequest` |
| GET | `/domains/{id}` | account | x | `api::domains::GetDomainRequest` |
| POST | `/domains` | account | x | `api::domains::CreateDomainRequest` |
| PUT | `/domains/{id}` | account | x | `api::domains::EditDomainRequest` |
| DELETE | `/domains/{id}` | account | x | `api::domains::DeleteDomainRequest` |
| POST | `/domains/{id}/verifyDKIM` | account | x | `api::domains::VerifyDkimRequest` |
| POST | `/domains/{id}/verifyreturnpath` | account | x | `api::domains::VerifyReturnPathRequest` |
| POST | `/domains/{id}/verifyspf` | account | x | `api::domains::VerifySpfRequest` |
| POST | `/domains/{id}/rotatedkim` | account | x | `api::domains::RotateDkimRequest` |

## Sender signatures

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/senders?count={count}&offset={offset}` | account | x | `api::signatures::ListSignaturesRequest` |
| GET | `/senders/{id}` | account | x | `api::signatures::GetSignatureRequest` |
| POST | `/senders` | account | x | `api::signatures::CreateSignatureRequest` |
| PUT | `/senders/{id}` | account | x | `api::signatures::EditSignatureRequest` |
| DELETE | `/senders/{id}` | account | x | `api::signatures::DeleteSignatureRequest` |
| POST | `/senders/{id}/resend` | account | x | `api::signatures::ResendSignatureConfirmationRequest` |
| POST | `/senders/{id}/verifyspf` | account | deprecated | - |
| POST | `/senders/{id}/requestnewdkim` | account | deprecated | - |

## Stats

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/stats/outbound` | server | x | `api::stats::GetOutboundOverviewRequest` |
| GET | `/stats/outbound/sends` | server | x | `api::stats::GetSentCountsRequest` |
| GET | `/stats/outbound/bounces` | server | x | `api::stats::GetBounceCountsRequest` |
| GET | `/stats/outbound/spam` | server | x | `api::stats::GetSpamComplaintsRequest` |
| GET | `/stats/outbound/tracked` | server | x | `api::stats::GetTrackedEmailCountsRequest` |
| GET | `/stats/outbound/opens` | server | x | `api::stats::GetEmailOpenCountsRequest` |
| GET | `/stats/outbound/opens/platforms` | server | x | `api::stats::GetEmailPlatformUsageRequest` |
| GET | `/stats/outbound/opens/emailclients` | server | x | `api::stats::GetEmailClientUsageRequest` |
| GET | `/stats/outbound/clicks` | server | x | `api::stats::GetClickCountsRequest` |
| GET | `/stats/outbound/clicks/browserfamilies` | server | x | `api::stats::GetBrowserUsageRequest` |
| GET | `/stats/outbound/clicks/platforms` | server | x | `api::stats::GetBrowserPlatformUsageRequest` |
| GET | `/stats/outbound/clicks/location` | server | x | `api::stats::GetClickLocationRequest` |

## Triggers: inbound rules

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/triggers/inboundrules?count={count}&offset={offset}` | server | x | `api::triggers::ListInboundRuleTriggersRequest` |
| POST | `/triggers/inboundrules` | server | x | `api::triggers::CreateInboundRuleTriggerRequest` |
| DELETE | `/triggers/inboundrules/{id}` | server | x | `api::triggers::DeleteInboundRuleTriggerRequest` |

## Webhooks

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/webhooks` | server | x | `api::webhooks::ListWebhooksRequest` |
| GET | `/webhooks/{id}` | server | x | `api::webhooks::GetWebhookRequest` |
| POST | `/webhooks` | server | x | `api::webhooks::CreateWebhookRequest` |
| PUT | `/webhooks/{id}` | server | x | `api::webhooks::EditWebhookRequest` |
| DELETE | `/webhooks/{id}` | server | x | `api::webhooks::DeleteWebhookRequest` |

## Data removal

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| POST | `/data-removals` | account | x | `api::data_removal::CreateDataRemovalRequest` |
| GET | `/data-removals/{id}` | account | x | `api::data_removal::GetDataRemovalStatusRequest` |

## Messages

| Method | Path | Token | Status | Rust request type |
|---|---|---|---|---|
| GET | `/messages/outbound` | server | x | `api::messages::OutboundSearchRequest` |
| GET | `/messages/outbound/{messageid}/details` | server | x | `api::messages::OutboundDetailsRequest` |
| GET | `/messages/outbound/{messageid}/dump` | server | x | `api::messages::OutboundDumpRequest` |
| GET | `/messages/inbound` | server | x | `api::messages::InboundSearchRequest` |
| GET | `/messages/inbound/{messageid}/details` | server | x | `api::messages::InboundDetailsRequest` |
| PUT | `/messages/inbound/{messageid}/bypass` | server | x | `api::messages::BypassBlockedInboundRequest` |
| PUT | `/messages/inbound/{messageid}/retry` | server | x | `api::messages::RetryFailedInboundRequest` |
| GET | `/messages/outbound/opens` | server | x | `api::messages::MessageOpensRequest` |
| GET | `/messages/outbound/opens/{messageid}` | server | x | `api::messages::SingleMessageOpensRequest` |
| GET | `/messages/outbound/clicks` | server | x | `api::messages::MessageClicksRequest` |
| GET | `/messages/outbound/clicks/{messageid}` | server | x | `api::messages::SingleMessageClicksRequest` |

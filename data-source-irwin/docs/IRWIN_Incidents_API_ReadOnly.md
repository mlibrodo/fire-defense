# IRWIN Incidents API v10 -- Read-Only Integration Notes

## Overview

The **Integrated Reporting of Wildfire Information (IRWIN)** API
provides a common method to exchange wildland fire incident data across
interagency systems.\

- Based on **ArcGIS REST Feature Services** with IRWIN-specific
  extensions.\
- Supports **Add**, **Query**, and **Update** operations.\
- Your system will typically only need **Query** with the `IRWINREAD`
  role.

---

## Credentials & Access

- **Formal Discovery Process Required**\
  You must complete the **IRWIN Discovery Process** with the IRWIN
  Core Team. This is how credentials are issued.\
  \> _"A formal discovery process is required to obtain an
  authentication credential... This document is not a replacement for
  that process."_

- **System-Level Accounts**\
  Each integrated system is issued a **system account** assigned to a
  role:

  - `IRWINREAD` → **read-only** queries (Incidents, Relationships,
    Resource Summary, Resources).\
  - `IRWINREADWRITE`, `IRWINCAD`, `IRWINFIREREPORTING` →
    write-enabled roles (not applicable for read-only).

---

## Environments

IRWIN provides three environments:

---

Environment Purpose URL

---

**TEST** For extended systems to `https://irwint.doi.gov/arcgis/rest/services`
test against released
software

**OAT** (Operational Acceptance QA testing against `https://irwinoat.doi.gov/arcgis/rest/services`
Testing) released software

**PROD** Production integration `https://irwin.doi.gov/arcgis/rest/services`
service

---

- `/next` folder exists in **TEST** and **OAT** for under-development
  versions.\
- Root (`/rest/services`) matches PROD stable release.

---

## Authentication

- **Token-based Authentication** (ArcGIS `generateToken` service)

  - Endpoint:

        {root}/tokens/generateToken

    (e.g., `https://irwin.doi.gov/arcgis/tokens/generateToken`)

- **Request Parameters:**

  ```text
  username=<your system username>
  password=<your system password>
  client=referer
  referer=<your referer string>
  expiration=60
  f=json
  ```

  - `client=referer` is **recommended**\
  - The `referer` value must match both:
    - The value supplied in the token request, and
    - The `Referer` HTTP header in subsequent API calls.

- **Token Lifetime:**

  - Maximum: **60 minutes**\
  - Best practice: request once per hour\
  - Do not request more than twice per hour

- **Response:** includes `token` and `expires` (epoch ms).

---

## Querying Incidents

- **Endpoint:**

      {base_services_url}/Irwin/Incidents/FeatureServer/0/query

  (layer `0` is the Incident layer)

- **Common Parameters:**

  - `f=json`\
  - `where=IsValid=1 AND IncidentTypeKind='FI'`\
  - `outFields=*` or a comma-separated list\
  - `returnGeometry=true`\
  - `token=<your token>`

- **IRWIN Extensions to ArcGIS Query:**

  - `includeADSStatus=true`\
  - `includeResources=true`\
  - `includeRelationships=true`\
  - `includeLastSyncDateTime=true`\
  - `includeFFR=true`

---

## Synchronization Pattern

- **ModifiedOnDateTime** is used for incremental sync.\
- Two approaches:
  1.  **IRWIN-driven sync**
      - Query with `where=ModifiedOnDateTime >= <lastSync>`\
      - IRWIN returns `nextSyncDateTime` in response.\
      - Use that as the cursor for the next query.\
  2.  **Client-driven sync**
      - Define explicit time windows in queries.

---

## Example Query

```http
GET https://irwin.doi.gov/arcgis/rest/services/Irwin/Incidents/FeatureServer/0/query
?f=json
&where=IsValid=1 AND IncidentTypeKind='FI'
&outFields=IrwinID,IncidentName,UniqueFireIdentifier,ModifiedOnDateTime
&returnGeometry=true
&includeResources=true
&includeRelationships=true
&token=<your-token>
```

---

## Roles Summary (for context)

- **IRWINREAD** → Query only\
- **IRWINREADWRITE** → Query, Add, Update (non-CAD)\
- **IRWINCAD** → Full CAD system role (adds more required fields)\
- **IRWINFIREREPORTING** → Fire reporting systems (manage FFR and
  certification)

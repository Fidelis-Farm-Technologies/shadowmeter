/*
 * Copyright 2024 Fidelis Farm & Technologies, LLC.
 * All Rights Reserved.
 * See license information in LICENSE.
 *
 * YAF file processor using fixbuf library.
 * See: https://tools.netsa.cert.org/fixbuf/libfixbuf/
 */
#define _GNU_SOURCE

#include <string.h>
#include <stdio.h>
#include <ndpi/ndpi_api.h>
#include <duckdb.h>
#include <maxminddb.h>
#include <fixbuf/public.h>
#include "fixbuf_wrapper.h"

#define CSV_OUTPUT_VERSION 100
#define CSV_OUTPUT_VERSION_EXT 101

#define GLIB_ERROR_RETURN(e)                         \
    {                                                \
        fprintf(stderr, "%s:%d: %s\n",               \
                __FUNCTION__, __LINE__, e->message); \
        return -1;                                   \
    }

#if defined(ENABLE_PROCESS_STATS)
static int processYafStatsRecord(const FILE *output_fp, const YAF_STATS_RECORD *yaf_stats_record)
{
    return 0;
}
#endif

#define FLOW_SCHEMA                                                                      \
    "CREATE TABLE flow ("                                                                \
    "observ VARCHAR,stime TIMESTAMP,etime TIMESTAMP,dur FLOAT,rtt FLOAT,proto UINTEGER," \
    "addr VARCHAR,raddr VARCHAR,port UINTEGER,rport UINTEGER,"                           \
    "iflag VARCHAR,riflag VARCHAR,"                                                      \
    "uflag VARCHAR,ruflag VARCHAR,"                                                      \
    "tcpseq VARCHAR,rtcpseq VARCHAR,"                                                    \
    "vlan UINTEGER,rvlan UINTEGER,"                                                      \
    "pkts UBIGINT,rpkts UBIGINT,"                                                        \
    "bytes UBIGINT,rbytes UBIGINT,"                                                      \
    "entropy UINTEGER,rentropy UINTEGER,"                                                \
    "data UBIGINT,rdata UBIGINT,"                                                        \
    "iat UBIGINT,riat UBIGINT,"                                                          \
    "stdev UBIGINT,rstdev UBIGINT,"                                                      \
    "tcpurg UINTEGER,rtcpurg UINTEGER,"                                                  \
    "smallpktcnt UINTEGER,rsmallpktcnt UINTEGER,"                                        \
    "nonemptypktcnt UINTEGER,rnonemptypktcnt UINTEGER,"                                  \
    "firstnonemptysize UINTEGER,rfirstnonemptysize UINTEGER,"                            \
    "stdevpayload UINTEGER,rstdevpayload UINTEGER,"                                      \
    "maxpktsize UINTEGER,rmaxpktsize UINTEGER,"                                          \
    "spd VARCHAR,appid VARCHAR,reason VARCHAR,"                                          \
    "mac VARCHAR,rmac VARCHAR,"                                                          \
    "oui VARCHAR,roui VARCHAR,"                                                          \
    "country VARCHAR,rcountry VARCHAR,"                                                  \
    "asn VARCHAR,rasn VARCHAR,"                                                          \
    "asnorg VARCHAR,rasnorg VARCHAR"                                                     \
    ")"

static void
print_tcp_flags(
    GString *str,
    uint8_t flags)
{
    if (flags & YF_TF_ECE)
    {
        g_string_append_c(str, 'E');
    }
    if (flags & YF_TF_CWR)
    {
        g_string_append_c(str, 'C');
    }
    if (flags & YF_TF_URG)
    {
        g_string_append_c(str, 'U');
    }
    if (flags & YF_TF_ACK)
    {
        g_string_append_c(str, 'A');
    }
    if (flags & YF_TF_PSH)
    {
        g_string_append_c(str, 'P');
    }
    if (flags & YF_TF_RST)
    {
        g_string_append_c(str, 'R');
    }
    if (flags & YF_TF_SYN)
    {
        g_string_append_c(str, 'S');
    }
    if (flags & YF_TF_FIN)
    {
        g_string_append_c(str, 'F');
    }
    if (!flags)
    {
        g_string_append_c(str, '0');
    }
}

static void print_ndpi(GString *rstr,
                       struct ndpi_detection_module_struct *ndpi_ctx,
                       uint16_t ndpi_master,
                       uint16_t ndpi_sub)
{
    char protocol_buf[128];
    ndpi_protocol protocol;
    protocol.master_protocol = ndpi_master;
    protocol.app_protocol = ndpi_sub;
    protocol.protocol_by_ip = 0;
    protocol.custom_category_userdata = NULL;
    char *appid = ndpi_protocol2name(ndpi_ctx, protocol, protocol_buf, sizeof(protocol_buf) - 1);

    g_string_append_printf(rstr, "%s", appid);
}

static int append_yaf_record(duckdb_appender appender,
                             const char *observation,
                             struct ndpi_detection_module_struct *ndpi_ctx,
                             const YAF_FLOW_RECORD *flow,
                             void *oui_db,
                             MMDB_s *asn_mmdb,
                             MMDB_s *country_mmdb)
{

    char sabuf[64], dabuf[64];
    GString *buffer = g_string_sized_new(256);

    duckdb_append_varchar(appender, observation);
    duckdb_timestamp start = {(flow->flowStartMilliseconds * 1000)};
    duckdb_append_timestamp(appender, start);
    duckdb_timestamp end = {(flow->flowEndMilliseconds * 1000)};
    duckdb_append_timestamp(appender, end);
    duckdb_append_float(appender, (flow->flowEndMilliseconds - flow->flowStartMilliseconds) / 1000.0);

    duckdb_append_float(appender, flow->reverseFlowDeltaMilliseconds / 1000.0);

    duckdb_append_uint8(appender, flow->protocolIdentifier);
    sabuf[0] = (char)0;
    dabuf[0] = (char)0;
    if (flow->sourceIPv4Address || flow->destinationIPv4Address)
    {
        air_ipaddr_buf_print(sabuf, flow->sourceIPv4Address);
        air_ipaddr_buf_print(dabuf, flow->destinationIPv4Address);
    }
    else
    {
        air_ip6addr_buf_print(sabuf, flow->sourceIPv6Address);
        air_ip6addr_buf_print(dabuf, flow->destinationIPv6Address);
    }

    duckdb_append_varchar(appender, sabuf);
    duckdb_append_varchar(appender, dabuf);
    duckdb_append_uint16(appender, flow->sourceTransportPort);
    duckdb_append_uint16(appender, flow->destinationTransportPort);

    /* print tcp flags */
    g_string_truncate(buffer, 0);
    print_tcp_flags(buffer, flow->initialTCPFlags);
    duckdb_append_varchar(appender, buffer->str);

    g_string_truncate(buffer, 0);
    print_tcp_flags(buffer, flow->reverseInitialTCPFlags);
    duckdb_append_varchar(appender, buffer->str);

    g_string_truncate(buffer, 0);
    print_tcp_flags(buffer, flow->unionTCPFlags);
    duckdb_append_varchar(appender, buffer->str);

    g_string_truncate(buffer, 0);
    print_tcp_flags(buffer, flow->reverseUnionTCPFlags);
    duckdb_append_varchar(appender, buffer->str);

    /* print tcp sequence numbers */
    duckdb_append_uint32(appender, flow->tcpSequenceNumber);
    duckdb_append_uint32(appender, flow->reverseTcpSequenceNumber);

    /* print vlan tags */
    if (flow->reverseOctetTotalCount)
    {
        duckdb_append_uint16(appender, flow->vlanId);
        duckdb_append_uint16(appender, flow->reverseVlanId);
    }
    else
    {
        duckdb_append_uint16(appender, flow->vlanId);
        duckdb_append_uint16(appender, 0);
    }

    /* print flow counters */
    duckdb_append_uint64(appender, flow->packetTotalCount);
    duckdb_append_uint64(appender, flow->reversePacketTotalCount);

    duckdb_append_uint64(appender, flow->octetTotalCount);
    duckdb_append_uint64(appender, flow->reverseOctetTotalCount);

    duckdb_append_uint8(appender, flow->entropy);
    duckdb_append_uint8(appender, flow->reverseEntropy);

    duckdb_append_uint64(appender, flow->dataByteCount);
    duckdb_append_uint64(appender, flow->reverseDataByteCount);

    duckdb_append_uint64(appender, flow->averageInterarrivalTime);
    duckdb_append_uint64(appender, flow->reverseAverageInterarrivalTime);

    duckdb_append_uint64(appender, flow->standardDeviationInterarrivalTime);
    duckdb_append_uint64(appender, flow->reverseStandardDeviationInterarrivalTime);

    duckdb_append_uint32(appender, flow->tcpUrgTotalCount);
    duckdb_append_uint32(appender, flow->reverseTcpUrgTotalCount);

    duckdb_append_uint32(appender, flow->smallPacketCount);
    duckdb_append_uint32(appender, flow->reverseSmallPacketCount);

    duckdb_append_uint32(appender, flow->nonEmptyPacketCount);
    duckdb_append_uint32(appender, flow->reverseNonEmptyPacketCount);

    duckdb_append_uint16(appender, flow->firstNonEmptyPacketSize);
    duckdb_append_uint16(appender, flow->reverseFirstNonEmptyPacketSize);

    duckdb_append_uint16(appender, flow->maxPacketSize);
    duckdb_append_uint16(appender, flow->reverseMaxPacketSize);

    duckdb_append_uint16(appender, flow->standardDeviationPayloadLength);
    duckdb_append_uint16(appender, flow->reverseStandardDeviationPayloadLength);

    g_string_truncate(buffer, 0);
    g_string_append_printf(buffer, "%c%c%c%c%c%c%c%c%s",
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 7)) ? '1' : '0'),
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 6)) ? '1' : '0'),
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 5)) ? '1' : '0'),
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 4)) ? '1' : '0'),
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 3)) ? '1' : '0'),
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 2)) ? '1' : '0'),
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 1)) ? '1' : '0'),
                           ((flow->firstEightNonEmptyPacketDirections & (1 << 0)) ? '1' : '0'),
                           YF_PRINT_DELIM);
    duckdb_append_varchar(appender, buffer->str);

    // ndpi
    g_string_truncate(buffer, 0);
    print_ndpi(buffer, ndpi_ctx, flow->ndpi_master, flow->ndpi_sub);
    duckdb_append_varchar(appender, buffer->str);

    /* end reason flags */
    g_string_truncate(buffer, 0);
    if ((flow->flowEndReason & YAF_END_MASK) == YAF_END_IDLE)
    {
        g_string_append(buffer, "idle");
    }
    if ((flow->flowEndReason & YAF_END_MASK) == YAF_END_ACTIVE)
    {
        g_string_append(buffer, "active");
    }
    if ((flow->flowEndReason & YAF_END_MASK) == YAF_END_FORCED)
    {
        g_string_append(buffer, "eof");
    }
    if ((flow->flowEndReason & YAF_END_MASK) == YAF_END_RESOURCE)
    {
        g_string_append(buffer, "rsrc");
    }
    if ((flow->flowEndReason & YAF_END_MASK) == YAF_END_UDPFORCE)
    {
        g_string_append(buffer, "force");
    }
    duckdb_append_varchar(appender, buffer->str);

    // smac
    g_string_truncate(buffer, 0);
    for (int loop = 0; loop < 6; loop++)
    {
        g_string_append_printf(buffer, "%02x", flow->sourceMacAddress[loop]);
        if (loop < 5)
        {
            g_string_append_printf(buffer, ":");
        }
    }
    duckdb_append_varchar(appender, buffer->str);

    // dmac
    g_string_truncate(buffer, 0);
    for (int loop = 0; loop < 6; loop++)
    {
        g_string_append_printf(buffer, "%02x",
                               flow->destinationMacAddress[loop]);
        if (loop < 5)
        {
            g_string_append_printf(buffer, ":");
        }
    }
    duckdb_append_varchar(appender, buffer->str);

    // if (label_oui)
    {
        duckdb_append_varchar(appender, "");
        duckdb_append_varchar(appender, "");
    }

    int gai_error, mmdb_error;
    MMDB_lookup_result_s result;
    char scountry[32] = {0};
    char dcountry[32] = {0};
    if (country_mmdb)
    {
        result = MMDB_lookup_string(country_mmdb, sabuf, &gai_error, &mmdb_error);
        if (gai_error)
        {
            fprintf(stderr, "%s: Country getaddrinfo failed: %s", __FUNCTION__, gai_strerror(gai_error));
        }
        else if (mmdb_error)
        {
            fprintf(stderr, "%s: Country geopip lookup failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
        }
        else if (result.found_entry)
        {
            MMDB_entry_data_s entry_data;
            if (MMDB_get_value(&result.entry, &entry_data,
                               "country", "iso_code", NULL) != MMDB_SUCCESS)
            {
                fprintf(stderr, "%s: MMDB_get_value failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
            }
            if (entry_data.has_data && entry_data.type == MMDB_DATA_TYPE_UTF8_STRING)
            {
                int len = entry_data.data_size > sizeof(scountry) ? sizeof(scountry) : entry_data.data_size;
                strncpy(scountry, entry_data.utf8_string, len);
                scountry[len] = '\0';
            }
        }

        result = MMDB_lookup_string(country_mmdb, dabuf, &gai_error, &mmdb_error);
        if (gai_error)
        {
            fprintf(stderr, "%s: Country getaddrinfo failed: %s", __FUNCTION__, gai_strerror(gai_error));
        }
        else if (mmdb_error)
        {
            fprintf(stderr, "%s: Country geopip lookup failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
        }
        else if (result.found_entry)
        {
            MMDB_entry_data_s entry_data;
            if (MMDB_get_value(&result.entry, &entry_data,
                               "country", "iso_code", NULL) != MMDB_SUCCESS)
            {
                fprintf(stderr, "%s: MMDB_get_value failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
            }
            if (entry_data.has_data && entry_data.type == MMDB_DATA_TYPE_UTF8_STRING)
            {
                int len = entry_data.data_size > sizeof(dcountry) ? sizeof(dcountry) : entry_data.data_size;
                strncpy(dcountry, entry_data.utf8_string, len);
                dcountry[len] = '\0';
            }
        }
    }
    duckdb_append_varchar(appender, scountry);
    duckdb_append_varchar(appender, dcountry);

    uint32_t sasn = 0;
    uint32_t dasn = 0;
    char sasnorg[128] = {0};
    char dasnorg[128] = {0};
    if (asn_mmdb)
    {
        result =
            MMDB_lookup_string(asn_mmdb, sabuf, &gai_error, &mmdb_error);
        if (gai_error)
        {
            fprintf(stderr, "%s: Country getaddrinfo failed: %s", __FUNCTION__, gai_strerror(gai_error));
        }
        else if (mmdb_error)
        {
            fprintf(stderr, "%s: Country geopip lookup failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
        }
        else if (result.found_entry)
        {
            MMDB_entry_data_s entry_data;
            if (MMDB_get_value(&result.entry, &entry_data,
                               "autonomous_system_number", NULL) != MMDB_SUCCESS)
            {
                fprintf(stderr, "%s: MMDB_get_value failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
            }
            if (entry_data.has_data && entry_data.type == MMDB_DATA_TYPE_UINT32)
            {
                sasn = entry_data.uint32;
            }
            if (MMDB_get_value(&result.entry, &entry_data,
                               "autonomous_system_organization", NULL) != MMDB_SUCCESS)
            {
                fprintf(stderr, "%s: MMDB_get_value failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
            }
            if (entry_data.has_data && entry_data.type == MMDB_DATA_TYPE_UTF8_STRING)
            {
                int len = entry_data.data_size > sizeof(sasnorg) ? sizeof(sasnorg) : entry_data.data_size;
                strncpy(sasnorg, entry_data.utf8_string, len);
                dasnorg[len] = '\0';
            }
        }

        result =
            MMDB_lookup_string(asn_mmdb, dabuf, &gai_error, &mmdb_error);
        if (gai_error)
        {
            fprintf(stderr, "%s: Country getaddrinfo failed: %s", __FUNCTION__, gai_strerror(gai_error));
        }
        else if (mmdb_error)
        {
            fprintf(stderr, "%s: Country geopip lookup failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
        }
        else if (result.found_entry)
        {
            MMDB_entry_data_s entry_data;
            if (MMDB_get_value(&result.entry, &entry_data,
                               "autonomous_system_number", NULL) != MMDB_SUCCESS)
            {
                fprintf(stderr, "%s: MMDB_get_value failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
            }
            if (entry_data.has_data && entry_data.type == MMDB_DATA_TYPE_UINT32)
            {
                dasn = entry_data.uint32;
            }
            if (MMDB_get_value(&result.entry, &entry_data,
                               "autonomous_system_organization", NULL) != MMDB_SUCCESS)
            {
                fprintf(stderr, "%s: MMDB_get_value failed: %s", __FUNCTION__, MMDB_strerror(mmdb_error));
            }
            if (entry_data.has_data && entry_data.type == MMDB_DATA_TYPE_UTF8_STRING)
            {
                int len = entry_data.data_size > sizeof(dasnorg) ? sizeof(dasnorg) : entry_data.data_size;
                strncpy(dasnorg, entry_data.utf8_string, len);
                dasnorg[len] = '\0';
            }
        }
    }

    duckdb_append_uint32(appender, sasn);
    duckdb_append_uint32(appender, dasn);
    duckdb_append_varchar(appender, sasnorg);
    duckdb_append_varchar(appender, dasnorg);

    /* release scratch buffers */
    g_string_free(buffer, TRUE);

    return 0;
}

static int process_yaf_record(const char *observation,
                              duckdb_appender appender,
                              struct ndpi_detection_module_struct *ndpi_ctx,
                              const YAF_FLOW_RECORD *flow,
                              void *label_oui,
                              MMDB_s *asn_mmdb,
                              MMDB_s *country_mmdb)
{
    int rc = 0;
    if (append_yaf_record(appender, observation, ndpi_ctx, flow, label_oui, asn_mmdb, country_mmdb) < 0)
    {
        rc = -1;
    }
    else
    {
        if (duckdb_appender_end_row(appender) == DuckDBError)
        {
            fprintf(stderr, "%s: %s\n", __FUNCTION__, duckdb_appender_error(appender));
        }
    }
    return rc;
}

int to_csv_file(const char *observation,
                const char *input_file,
                const char *output_dir,
                const char *archive_dir,
                const char *oui_db,
                const char *asn_db,
                const char *country_db)
{

    fBuf_t *fbuf;
    YAF_FLOW_RECORD yaf_record;
    size_t yaf_rec_len = sizeof(yaf_record);
    GError *err = NULL;
    FILE *input_fp = NULL;
    duckdb_database db;
    duckdb_connection con;
    duckdb_appender appender;
    char *yaf_file_basename = NULL;
    char output_file[PATH_MAX];
    fbCollector_t *collector;
    size_t record_count = 0;
    struct ndpi_detection_module_struct *ndpi_ctx = ndpi_init_detection_module(0);

    //
    // initialize ndpi
    //
    if (ndpi_ctx == NULL)
    {
        fprintf(stderr, "ndpi_init_detection_module() failed\n");
        return -1;
    }

    NDPI_PROTOCOL_BITMASK protos;
    NDPI_BITMASK_SET_ALL(protos);
    ndpi_set_protocol_detection_bitmask2(ndpi_ctx, &protos);
    ndpi_finalize_initialization(ndpi_ctx);

    //
    // maxmind ASN
    //
    MMDB_s asn_mmdb;
    MMDB_s *asn_mmdb_ptr = NULL;
    memset(&asn_mmdb, 0, sizeof(asn_mmdb));
    if (asn_db && strlen(asn_db))
    {
        if (MMDB_SUCCESS != MMDB_open(asn_db, MMDB_MODE_MMAP, &asn_mmdb))
        {
            fprintf(stderr, "failed to load geolite - asn: %s\n", asn_db);
            return -1;
        }
        asn_mmdb_ptr = &asn_mmdb;
    }

    //
    // maxmind Country
    //
    MMDB_s country_mmdb;
    MMDB_s *country_mmdb_ptr = NULL;
    memset(&country_mmdb, 0, sizeof(country_mmdb));
    if (country_db && strlen(country_db))
    {
        if (MMDB_SUCCESS != MMDB_open(country_db, MMDB_MODE_MMAP, &country_mmdb))
        {
            fprintf(stderr, "failed to load geolite - country: %s\n", country_db);
            return -1;
        }
        country_mmdb_ptr = &country_mmdb;
    }

    memset(&yaf_record, 0, yaf_rec_len);

    fbInfoModel_t *model = fbInfoModelAlloc();
    if (model == NULL)
        GLIB_ERROR_RETURN(err);

    fbInfoModelAddElementArray(model, yaf_enterprise_elements);

    fbSession_t *session = fbSessionAlloc(model);
    if (session == NULL)
        GLIB_ERROR_RETURN(err);

    fbTemplate_t *tmpl = fbTemplateAlloc(model);
    if (tmpl == NULL)
        GLIB_ERROR_RETURN(err);

    if (fbTemplateAppendSpecArray(tmpl, yaf_flow_spec, YTF_ALL, &err) == FALSE)
        GLIB_ERROR_RETURN(err);

    if (!fbSessionAddTemplate(session, TRUE, YAF_FLOW_FULL_TID, tmpl, NULL, &err))
        GLIB_ERROR_RETURN(err);

    if (input_file && strlen(input_file))
    {
        if (strncmp(input_file, "stdin", 5) == 0)
        {
            collector = fbCollectorAllocFP(NULL, stdin);
        }
        else
        {
            yaf_file_basename = basename(input_file);
            input_fp = fopen(input_file, "rb");
            if (input_fp == NULL)
            {
                fprintf(stderr, "%s: error opening %s\n", __FUNCTION__, input_file);
                return -1;
            }
            collector = fbCollectorAllocFP(NULL, input_fp);
        }
    }
    else
    {
        fprintf(stderr, "missing input specifier\n");
        return -1;
    }
    printf("processing: %s\n", input_file);

    if (collector == NULL)
        GLIB_ERROR_RETURN(err);

    if (output_dir && strlen(output_dir))
    {
        snprintf(output_file, sizeof(output_file) - 1, "%s/%s.flow", output_dir, yaf_file_basename);
        //
        // initialize duckdb
        //
        duckdb_config config;

        // create the configuration object
        if (duckdb_create_config(&config) == DuckDBError)
        {
            // handle error
        }
        // set some configuration options
        duckdb_set_config(config, "access_mode", "READ_WRITE"); // or READ_ONLY
        duckdb_set_config(config, "threads", "4");
        duckdb_set_config(config, "max_memory", "8GB");
        duckdb_set_config(config, "default_order", "DESC");

        // open the database using the configuration
        if (duckdb_open_ext(output_file, &db, config, NULL) == DuckDBError)
        {
            fprintf(stderr, "%s: error opening %s\n", __FUNCTION__, output_file);
            return -1;
        }
        // cleanup the configuration object
        duckdb_destroy_config(&config);

        if (duckdb_connect(db, &con) == DuckDBError)
        {
            fprintf(stderr, "%s: error connecting %s\n", __FUNCTION__, output_file);
            return -1;
        }

        duckdb_result result;
        if (duckdb_query(con, FLOW_SCHEMA, &result) == DuckDBError)
        {
            fprintf(stderr, "%s: error generating schema: \n%s\n", __FUNCTION__, duckdb_result_error(&result));
            return -1;
        }

        if (duckdb_appender_create(con, NULL, "flow", &appender) == DuckDBError)
        {
            fprintf(stderr, "%s: error creating appender %s\n", __FUNCTION__, output_file);
            return -1;
        }
    }
    else
    {
        fprintf(stderr, "missing output specifier\n");
        return -1;
    }

    fbuf = fBufAllocForCollection(session, collector);
    if (fbuf == NULL)
        GLIB_ERROR_RETURN(err);

    if (!fBufSetInternalTemplate(fbuf, YAF_FLOW_FULL_TID, &err))
        GLIB_ERROR_RETURN(err);

    while (fBufNext(fbuf, (uint8_t *)&yaf_record, &yaf_rec_len, &err))
    {
        if (process_yaf_record(observation, appender, ndpi_ctx, &yaf_record, NULL, asn_mmdb_ptr, country_mmdb_ptr) < 0)
        {
            fprintf(stderr, "error: %s", strerror(errno));
            return -1;
        }
        record_count++;
    }

    printf("processed %lu records\n", record_count);
    duckdb_appender_flush(appender);
    duckdb_appender_destroy(&appender);
    duckdb_disconnect(&con);
    duckdb_close(&db);

    if (asn_db && strlen(asn_db))
    {
        MMDB_close(&asn_mmdb);
    }

    if (country_db && strlen(country_db))
    {
        MMDB_close(&country_mmdb);
    }

    if (!g_error_matches(err, FB_ERROR_DOMAIN, FB_ERROR_EOF))
        GLIB_ERROR_RETURN(err);
    g_clear_error(&err);

    //  This frees the Buffer, Session, Templates, and Collector.
    fBufFree(fbuf);
    fbInfoModelFree(model);

    if (input_fp)
        fclose(input_fp);

    if (archive_dir && strlen(archive_dir))
    {
        char archive_path[PATH_MAX];
        snprintf(archive_path, sizeof(archive_path) - 1, "%s/%s", archive_dir, yaf_file_basename);
        printf("archiving: %s\n", archive_path);

        if (rename(input_file, archive_path) != 0)
        {
            fprintf(stderr, "%s: error archiving %s\n", __FUNCTION__, archive_path);
            return -1;
        }
    }
    return record_count;
}

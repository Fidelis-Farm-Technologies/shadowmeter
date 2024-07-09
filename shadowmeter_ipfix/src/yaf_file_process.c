/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

#include <stdio.h>
#include <fixbuf/public.h>

#define FATAL(e)                                 \
    {                                            \
        fprintf(stderr, "Failed at %s:%d: %s\n", \
                __FILE__, __LINE__, e->message); \
        exit(1);                                 \
    }

fbInfoElementSpec_t collectTemplate[] = {
    {"flowStartMilliseconds", 8, 0},
    {"flowEndMilliseconds", 8, 0},
    {"sourceIPv4Address", 4, 0},
    {"destinationIPv4Address", 4, 0},
    {"sourceTransportPort", 2, 0},
    {"destinationTransportPort", 2, 0},
    {"protocolIdentifier", 1, 0},
    {"paddingOctets", 3, 0},
    {"packetTotalCount", 8, 0},
    {"octetTotalCount", 8, 0},
    {"ipPayloadPacketSection", 0, 0},
    FB_IESPEC_NULL};

struct collectRecord_st
{
    uint64_t flowStartMilliseconds;
    uint64_t flowEndMilliseconds;
    uint32_t sourceIPv4Address;
    uint32_t destinationIPv4Address;
    uint16_t sourceTransportPort;
    uint16_t destinationTransportPort;
    uint8_t protocolIdentifier;
    uint8_t padding[3];
    uint64_t packetTotalCount;
    uint64_t octetTotalCount;
    fbVarfield_t payload;
} collectRecord;

int yaf2csv(const char *input_file, const char *output_file)
{
    printf("%s: input: %s, output: %s\n", __FUNCTION__, input_file, output_file);

    fbInfoModel_t *model;
    fbSession_t *session;
    fbCollector_t *collector;
    fbTemplate_t *tmpl;
    fBuf_t *fbuf;
    uint16_t tid;
    size_t reclen;
    GError *err = NULL;

    memset(&collectRecord, 0, sizeof(collectRecord));

    model = fbInfoModelAlloc();
    //  Use if needed to define elements used by YAF.
    // if (!fbInfoModelReadXMLFile(model, "cert_ipfix.xml", &err))
    //    FATAL(err);

    session = fbSessionAlloc(model);

    tmpl = fbTemplateAlloc(model);
    if (!fbTemplateAppendSpecArray(tmpl, collectTemplate, ~0, &err))
        FATAL(err);
    if (!(tid = fbSessionAddTemplate(
              session, TRUE, FB_TID_AUTO, tmpl, NULL, &err)))
        FATAL(err);

    collector = fbCollectorAllocFP(NULL, stdin);
    fbuf = fBufAllocForCollection(session, collector);

    if (!fBufSetInternalTemplate(fbuf, tid, &err))
        FATAL(err);
    
    printf("%s: processing\n", __FUNCTION__);
    reclen = sizeof(collectRecord);
    while (fBufNext(fbuf, (uint8_t *)&collectRecord, &reclen, &err))
    {
        //std::cout << "processed record" << std::endl;
    }
    printf("%s: done\n", __FUNCTION__);    
    if (!g_error_matches(err, FB_ERROR_DOMAIN, FB_ERROR_EOF))
        FATAL(err);
    g_clear_error(&err);

    //  This frees the Buffer, Session, Templates, and Collector.
    fBufFree(fbuf);
    fbInfoModelFree(model);

    return 0;
}
int yaf2json(const char *input_file, const char *output_file)
{
    printf("%s: input: %s, output: %s\n", __FUNCTION__, input_file, output_file);

    // not implemented yet
    return -1;
}

/*
int main()
{

    std::cout << "yaf_process" << std::endl;


    fbInfoModel_t *model;
    fbSession_t *session;
    fbCollector_t *collector;
    fbTemplate_t *tmpl;
    fBuf_t *fbuf;
    uint16_t tid;
    size_t reclen;
    GError *err = NULL;

    memset(&collectRecord, 0, sizeof(collectRecord));

    model = fbInfoModelAlloc();
    //  Use if needed to define elements used by YAF.
    // if (!fbInfoModelReadXMLFile(model, "cert_ipfix.xml", &err))
    //    FATAL(err);

    session = fbSessionAlloc(model);

    tmpl = fbTemplateAlloc(model);
    if (!fbTemplateAppendSpecArray(tmpl, collectTemplate, ~0, &err))
        FATAL(err);
    if (!(tid = fbSessionAddTemplate(
              session, TRUE, FB_TID_AUTO, tmpl, NULL, &err)))
        FATAL(err);

    collector = fbCollectorAllocFP(NULL, stdin);
    fbuf = fBufAllocForCollection(session, collector);

    if (!fBufSetInternalTemplate(fbuf, tid, &err))
        FATAL(err);

    reclen = sizeof(collectRecord);
    while (fBufNext(fbuf, (uint8_t *)&collectRecord, &reclen, &err))
    {
        std::cout << "processed record" << std::endl;
    }
    if (!g_error_matches(err, FB_ERROR_DOMAIN, FB_ERROR_EOF))
        FATAL(err);
    g_clear_error(&err);

    //  This frees the Buffer, Session, Templates, and Collector.
    fBufFree(fbuf);
    fbInfoModelFree(model);

    return 0;
}
*/
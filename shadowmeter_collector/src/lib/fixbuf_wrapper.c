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
#include <fixbuf/public.h>
#include "fixbuf_wrapper.h"

#define GLIB_ERROR_RETURN(e)                         \
    {                                                \
        fprintf(stderr, "%s:%d: %s\n",               \
                __FUNCTION__, __LINE__, e->message); \
        exit (-1);                                   \
    }

#if defined(ENABLE_PROCESS_STATS)
static int processYafStatsRecord(const FILE *output_fp, const YAF_STATS_RECORD *yaf_stats_record) {
    return 0;
}
#endif

static int processYafFlowRecord(const FILE *output_fp, const YAF_FLOW_RECORD *yaf_flow_record) {
    return 0;
}

int yaf2csv(const char *input_file, const char *output_dir, const char *archive_dir)
{
    printf("%s: input: %s, output: %s\n", __FUNCTION__, input_file, output_dir);

    fBuf_t *fbuf;
    YAF_FLOW_RECORD yaf_record;    
    size_t yaf_rec_len = sizeof(yaf_record);
    GError *err = NULL;
    FILE *input_fp = NULL;
    FILE *output_fp = NULL;    
    char *yaf_file_basename = NULL;
    char output_file[PATH_MAX];
    fbCollector_t *collector;    
    size_t record_count = 0;
 
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
    

    if (input_file && strlen(input_file)) {
        yaf_file_basename = basename(input_file);
        input_fp = fopen(input_file, "rb");
        if (input_fp == NULL) {
            fprintf(stderr, "%s: error opening %s\n", __FUNCTION__, input_file);
            return -1;
        }        
        collector = fbCollectorAllocFP(NULL, input_fp);
    }
    else {
        collector = fbCollectorAllocFP(NULL, stdin);
    }

    if (collector == NULL)
        GLIB_ERROR_RETURN(err);

    if (input_fp && output_dir && strlen(output_dir)) {
        snprintf(output_file,sizeof (output_file)-1, "%s/%s.csv", output_dir, yaf_file_basename);
        output_fp = fopen(output_file, "w");
        if (output_fp == NULL) {
            fprintf(stderr, "%s: error opening %s\n", __FUNCTION__, output_file);
            return -1;
        }
    }

    fbuf = fBufAllocForCollection(session, collector);
    if (fbuf ==NULL) 
        GLIB_ERROR_RETURN(err);

    if (!fBufSetInternalTemplate(fbuf, YAF_FLOW_FULL_TID, &err))
        GLIB_ERROR_RETURN(err);

    //
    //
    //
    printf("%s: processing\n", __FUNCTION__);
    while (fBufNext(fbuf, (uint8_t *)&yaf_record, &yaf_rec_len, &err)) {
        if (processYafFlowRecord (output_fp, &yaf_record) == 0)
            record_count++;
    }
    printf("%s: processed %lu records\n", __FUNCTION__, record_count);

    if (!g_error_matches(err, FB_ERROR_DOMAIN, FB_ERROR_EOF))
        GLIB_ERROR_RETURN(err);
    g_clear_error(&err);

    //  This frees the Buffer, Session, Templates, and Collector.
    fBufFree(fbuf);
    fbInfoModelFree(model);

    if (input_fp)
        fclose(input_fp);

    if (output_fp)
        fclose(output_fp);


    if (archive_dir && strlen(archive_dir)) {
        char archive_path[PATH_MAX];
        snprintf(archive_path, sizeof (archive_path) -1, "%s/%s", archive_dir, yaf_file_basename);
       
        if (rename(input_file, archive_path) != 0) {
            fprintf(stderr, "%s: error archiving %s\n", __FUNCTION__, archive_path);
            return -1;
        }
        else {
             printf("%s: archived %s\n", __FUNCTION__, archive_path);
        }
    }
    return 0;
}
int yaf2json(const char *input_file, const char *output_file, const char *archive_dir)
{
    fprintf(stderr, "%s:%s: not implemented yet\n", __FILE__, __FUNCTION__);
    return -1;
}

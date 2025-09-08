-- GDPR Data Subject Rights Tables
-- Auto-generated migration for GDPR compliance

-- Data Subject Requests table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE gdpr_data_subject_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    request_type VARCHAR(50) NOT NULL CHECK (request_type IN (
        'data_access', 'data_rectification', 'data_erasure', 
        'data_portability', 'processing_restriction', 'object_to_processing',
        'consent_withdrawal'
    )),
    subject_id VARCHAR(255) NOT NULL,
    subject_email VARCHAR(255),
    verification_status VARCHAR(30) NOT NULL DEFAULT 'pending' CHECK (verification_status IN (
        'pending', 'verified', 'failed', 'requires_manual_review'
    )),
    status VARCHAR(30) NOT NULL DEFAULT 'submitted' CHECK (status IN (
        'submitted', 'under_review', 'in_progress', 'completed', 
        'rejected', 'partially_completed'
    )),
    reference_number VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    verification_data JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    deadline TIMESTAMP WITH TIME ZONE NOT NULL,
    rejection_reason TEXT,
    completion_notes TEXT,
    
    -- Audit fields
    created_by UUID,
    assigned_to UUID,
    ip_address INET,
    user_agent TEXT,
    
    -- Indexes for performance
    INDEX idx_gdpr_requests_subject_id (subject_id),
    INDEX idx_gdpr_requests_status (status),
    INDEX idx_gdpr_requests_type (request_type),
    INDEX idx_gdpr_requests_created_at (created_at),
    INDEX idx_gdpr_requests_deadline (deadline)
);

-- Data Subject Request Activities (audit trail for request processing)
CREATE TABLE gdpr_request_activities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    request_id UUID NOT NULL REFERENCES gdpr_data_subject_requests(id) ON DELETE CASCADE,
    activity_type VARCHAR(50) NOT NULL,
    description TEXT NOT NULL,
    performed_by UUID,
    performed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    metadata JSONB,
    
    INDEX idx_gdpr_activities_request_id (request_id),
    INDEX idx_gdpr_activities_type (activity_type),
    INDEX idx_gdpr_activities_performed_at (performed_at)
);

-- Data Export Files (for data portability requests)
CREATE TABLE gdpr_data_exports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    request_id UUID NOT NULL REFERENCES gdpr_data_subject_requests(id) ON DELETE CASCADE,
    format VARCHAR(10) NOT NULL CHECK (format IN ('json', 'csv', 'xml', 'pdf')),
    file_path VARCHAR(500) NOT NULL,
    file_size BIGINT,
    checksum VARCHAR(64),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    downloaded_at TIMESTAMP WITH TIME ZONE,
    download_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    INDEX idx_gdpr_exports_request_id (request_id),
    INDEX idx_gdpr_exports_expires_at (expires_at)
);

-- Data Deletion Records (for right to erasure tracking)
CREATE TABLE gdpr_data_deletions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    request_id UUID NOT NULL REFERENCES gdpr_data_subject_requests(id) ON DELETE CASCADE,
    table_name VARCHAR(100) NOT NULL,
    record_identifier VARCHAR(255) NOT NULL,
    deletion_method VARCHAR(20) NOT NULL CHECK (deletion_method IN (
        'soft_delete', 'hard_delete', 'anonymization', 'pseudonymization'
    )),
    deleted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    backup_location VARCHAR(500),
    metadata JSONB,
    
    INDEX idx_gdpr_deletions_request_id (request_id),
    INDEX idx_gdpr_deletions_table (table_name),
    INDEX idx_gdpr_deletions_deleted_at (deleted_at)
);

-- Consent Records (GDPR Article 7 compliance)
CREATE TABLE gdpr_consent_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id VARCHAR(255) NOT NULL,
    consent_category VARCHAR(100) NOT NULL,
    purpose TEXT NOT NULL,
    lawful_basis VARCHAR(30) NOT NULL CHECK (lawful_basis IN (
        'consent', 'contract', 'legal_obligation', 'vital_interests', 
        'public_task', 'legitimate_interests'
    )),
    granted BOOLEAN NOT NULL,
    granted_at TIMESTAMP WITH TIME ZONE,
    withdrawn_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    consent_method VARCHAR(20) NOT NULL CHECK (consent_method IN (
        'web_form', 'email', 'phone', 'in_person', 'api', 'implied'
    )),
    consent_text TEXT,
    version INTEGER NOT NULL DEFAULT 1,
    ip_address INET,
    user_agent TEXT,
    metadata JSONB,
    
    -- Ensure only one active consent per subject/category
    UNIQUE (subject_id, consent_category, version),
    
    INDEX idx_gdpr_consent_subject_id (subject_id),
    INDEX idx_gdpr_consent_category (consent_category),
    INDEX idx_gdpr_consent_granted_at (granted_at),
    INDEX idx_gdpr_consent_expires_at (expires_at)
);

-- Audit Events (comprehensive audit trail)
CREATE TABLE gdpr_audit_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_type VARCHAR(50) NOT NULL,
    subject_id VARCHAR(255),
    user_id UUID,
    table_name VARCHAR(100),
    record_id VARCHAR(255),
    old_values JSONB,
    new_values JSONB,
    ip_address INET,
    user_agent TEXT,
    session_id VARCHAR(100),
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    metadata JSONB,
    
    INDEX idx_gdpr_audit_event_type (event_type),
    INDEX idx_gdpr_audit_subject_id (subject_id),
    INDEX idx_gdpr_audit_user_id (user_id),
    INDEX idx_gdpr_audit_table_name (table_name),
    INDEX idx_gdpr_audit_timestamp (timestamp)
);

-- Data Classification (track what data is personal/sensitive)
CREATE TABLE gdpr_data_classification (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    table_name VARCHAR(100) NOT NULL,
    column_name VARCHAR(100) NOT NULL,
    classification_type VARCHAR(30) NOT NULL CHECK (classification_type IN (
        'personal_data', 'sensitive_personal_data', 'financial_data',
        'health_data', 'technical_data', 'behavioral_data',
        'contact_information', 'identification_data'
    )),
    sensitivity_level VARCHAR(20) NOT NULL CHECK (sensitivity_level IN (
        'public', 'internal', 'confidential', 'restricted'
    )),
    retention_period VARCHAR(20),
    encryption_required BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    UNIQUE (table_name, column_name),
    INDEX idx_gdpr_classification_table (table_name),
    INDEX idx_gdpr_classification_type (classification_type)
);

-- Data Processing Activities (Article 30 record of processing)
CREATE TABLE gdpr_processing_activities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    activity_name VARCHAR(200) NOT NULL,
    purpose TEXT NOT NULL,
    lawful_basis VARCHAR(30) NOT NULL,
    data_categories TEXT[] NOT NULL,
    data_subjects_categories TEXT[] NOT NULL,
    recipients TEXT[],
    third_country_transfers BOOLEAN DEFAULT FALSE,
    retention_period VARCHAR(50),
    security_measures TEXT[],
    controller_name VARCHAR(200) NOT NULL,
    controller_contact VARCHAR(200) NOT NULL,
    dpo_contact VARCHAR(200),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    INDEX idx_gdpr_processing_name (activity_name),
    INDEX idx_gdpr_processing_purpose (purpose)
);

-- Breach Incidents (Articles 33-34 compliance)
CREATE TABLE gdpr_breach_incidents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    incident_reference VARCHAR(50) UNIQUE NOT NULL,
    detected_at TIMESTAMP WITH TIME ZONE NOT NULL,
    reported_at TIMESTAMP WITH TIME ZONE,
    breach_type VARCHAR(50) NOT NULL,
    affected_records_count INTEGER,
    affected_subjects TEXT[],
    breach_description TEXT NOT NULL,
    consequences TEXT,
    measures_taken TEXT,
    supervisory_authority_notified BOOLEAN DEFAULT FALSE,
    subjects_notified BOOLEAN DEFAULT FALSE,
    notification_deadline TIMESTAMP WITH TIME ZONE,
    risk_level VARCHAR(20) CHECK (risk_level IN ('low', 'medium', 'high')),
    status VARCHAR(30) DEFAULT 'investigating' CHECK (status IN (
        'investigating', 'contained', 'resolved', 'escalated'
    )),
    created_by UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    INDEX idx_gdpr_breach_detected_at (detected_at),
    INDEX idx_gdpr_breach_status (status),
    INDEX idx_gdpr_breach_risk_level (risk_level)
);

-- Triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_gdpr_requests_updated_at BEFORE UPDATE ON gdpr_data_subject_requests FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_gdpr_classification_updated_at BEFORE UPDATE ON gdpr_data_classification FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_gdpr_processing_updated_at BEFORE UPDATE ON gdpr_processing_activities FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_gdpr_breach_updated_at BEFORE UPDATE ON gdpr_breach_incidents FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to automatically log audit events
CREATE OR REPLACE FUNCTION log_gdpr_audit_event()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO gdpr_audit_events (
        event_type,
        table_name,
        record_id,
        old_values,
        new_values,
        timestamp
    ) VALUES (
        TG_OP,
        TG_TABLE_NAME,
        COALESCE(NEW.id::text, OLD.id::text),
        CASE WHEN TG_OP = 'DELETE' THEN to_jsonb(OLD) ELSE NULL END,
        CASE WHEN TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN to_jsonb(NEW) ELSE NULL END,
        NOW()
    );
    
    RETURN COALESCE(NEW, OLD);
END;
$$ language 'plpgsql';

-- Add audit triggers to sensitive tables (can be customized per application)
-- These will be auto-generated based on data classification

-- Cleanup job for expired export files
CREATE OR REPLACE FUNCTION cleanup_expired_exports()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM gdpr_data_exports 
    WHERE expires_at < NOW();
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ language 'plpgsql';

-- View for GDPR compliance reporting
CREATE VIEW gdpr_compliance_summary AS
SELECT 
    DATE_TRUNC('month', created_at) as month,
    request_type,
    status,
    COUNT(*) as request_count,
    AVG(EXTRACT(EPOCH FROM (COALESCE(completed_at, NOW()) - created_at))/86400) as avg_processing_days,
    COUNT(*) FILTER (WHERE completed_at <= deadline) as within_deadline_count,
    COUNT(*) FILTER (WHERE completed_at > deadline OR (deadline < NOW() AND completed_at IS NULL)) as overdue_count
FROM gdpr_data_subject_requests
GROUP BY DATE_TRUNC('month', created_at), request_type, status
ORDER BY month DESC, request_type;

-- Grant necessary permissions (adjust as needed)
-- GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO gdpr_service_user;
-- GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO gdpr_service_user;
CREATE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON products
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

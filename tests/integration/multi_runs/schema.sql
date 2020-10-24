--
-- PostgreSQL database dump
--

-- Dumped from database version 10.12 (Debian 10.12-2.pgdg90+1)
-- Dumped by pg_dump version 13.0

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA public;


--
-- Name: db_changelog; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.db_changelog (
    id bigint NOT NULL,
    execution_order integer NOT NULL,
    checksum character varying(44) NOT NULL,
    created_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


--
-- Name: db_changelog_execution_order_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.db_changelog_execution_order_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: db_changelog_execution_order_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.db_changelog_execution_order_seq OWNED BY public.db_changelog.execution_order;


--
-- Name: test_table; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.test_table (
);


--
-- Name: test_table2; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.test_table2 (
);


--
-- Name: test_table3; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.test_table3 (
);


--
-- Name: test_table4; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.test_table4 (
);


--
-- Name: db_changelog execution_order; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.db_changelog ALTER COLUMN execution_order SET DEFAULT nextval('public.db_changelog_execution_order_seq'::regclass);


--
-- Name: db_changelog db_changelog_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.db_changelog
    ADD CONSTRAINT db_changelog_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--


-- TABLAS

CREATE TABLE BACKO.NEW_TABLE 
    ( 
     AGUA_ACTUAL NUMBER , 
     ID          NUMBER  NOT NULL , 
     FECHA       DATE  NOT NULL 
    ) 
    TABLESPACE DATA 
    LOGGING 
;

CREATE TABLE BACKO.NEW_TABLE 
    ( 
     ID             NUMBER  NOT NULL , 
     AMBITO_NOMBRE  VARCHAR2 (4000) , 
     EMBALSE_NOMBRE VARCHAR2 (4000)  NOT NULL , 
     AGUA_TOTAL     NUMBER , 
     ELECTRICO_FLAG NUMBER 
    ) 
    TABLESPACE DATA 
    LOGGING 
;

CREATE TABLE BACKO.NEW_TABLE 
    ( 
     CODIGO        NUMBER (38)  NOT NULL , 
     NOMBRE        VARCHAR2 (512)  NOT NULL , 
     EMBALSE       VARCHAR2 (512) , 
     X             NUMBER (38,13) , 
     Y             NUMBER (38,15) , 
     DEMARC        VARCHAR2 (512) , 
     CAUCE         VARCHAR2 (512) , 
     GOOGLE        VARCHAR2 (4000) , 
     OPENSTREETMAP VARCHAR2 (4000) , 
     WIKIDATA      VARCHAR2 (4000) , 
     PROVINCIA     VARCHAR2 (512) , 
     CCAA          VARCHAR2 (512) , 
     TIPO          VARCHAR2 (4000) , 
     COTA_CORON    NUMBER (38,12) , 
     ALT_CIMIEN    NUMBER (38,13) , 
     INFORME       VARCHAR2 (4000) 
    ) 
    TABLESPACE DATA 
    LOGGING 
;

-- FUNCIONES

create or replace FUNCTION f_get_historico_agua(p_embalse_nombre IN VARCHAR2)
RETURN SYS_REFCURSOR IS
  v_result SYS_REFCURSOR;
BEGIN
  OPEN v_result FOR
    SELECT e.ID, e.ambito_nombre, e.agua_total, a.fecha, a.agua_actual
    FROM backo.embalse e
    FULL OUTER JOIN backo.agua a
    ON e.id = a.id
    WHERE e.embalse_nombre = p_embalse_nombre
    ORDER BY a.fecha ASC;
  RETURN v_result;
END f_get_historico_agua;



create or replace FUNCTION f_get_intervalo_historico_agua(
    p_embalse_nombre IN VARCHAR2,
    p_fecha_inicio IN DATE,
    p_fecha_fin IN DATE
) RETURN NUMBER IS
  v_result SYS_REFCURSOR;
  v_id NUMBER;
  v_ambito_nombre VARCHAR2(100);
  v_agua_total NUMBER;
  v_fecha DATE;
  v_agua_actual NUMBER;
  v_maximo_agua_actual NUMBER := 0;
BEGIN
    -- Ejecutamos la funciÃ³n y almacenamos el cursor en la variable 'v_result'
    v_result := f_get_historico_agua(p_embalse_nombre);

    -- Iteramos sobre los resultados del cursor
    LOOP
        FETCH v_result INTO v_id, v_ambito_nombre, v_agua_total, v_fecha, v_agua_actual;
        EXIT WHEN v_result%NOTFOUND;

        -- Filtramos los resultados por las fechas proporcionadas
        IF v_fecha >= p_fecha_inicio AND v_fecha <= p_fecha_fin THEN
            -- Si el valor actual de agua es mayor al mÃ¡ximo actual
            IF v_agua_actual > v_maximo_agua_actual THEN
                v_maximo_agua_actual := v_agua_actual;
            END IF;
        END IF;
    END LOOP;

    -- Cerramos el cursor
    CLOSE v_result;

    -- Retornamos el valor mÃ¡ximo de agua actual
    RETURN v_maximo_agua_actual;
END f_get_intervalo_historico_agua;


create or replace FUNCTION f_get_maximo_historico_agua(p_embalse_nombre IN VARCHAR2)
RETURN NUMBER IS
  v_result SYS_REFCURSOR;
  v_id NUMBER;
  v_ambito_nombre VARCHAR2(100);
  v_agua_total NUMBER;
  v_fecha DATE;
  v_agua_actual NUMBER;
  v_maximo_agua_actual NUMBER := 0;
BEGIN
    -- Ejecutamos la funciÃ³n y almacenamos el cursor en la variable 'v_result'
    v_result := f_get_historico_agua(p_embalse_nombre);

    -- Iteramos sobre los resultados del cursor
    LOOP
        FETCH v_result INTO v_id, v_ambito_nombre, v_agua_total, v_fecha, v_agua_actual;
        EXIT WHEN v_result%NOTFOUND;

        -- Si el valor actual de agua es mayor al mÃ¡ximo actual
        IF v_agua_actual > v_maximo_agua_actual THEN
            v_maximo_agua_actual := v_agua_actual;
        END IF;
    END LOOP;

    -- Cerramos el cursor
    CLOSE v_result;

    -- Retornamos el valor mÃ¡ximo de agua actual
    RETURN v_maximo_agua_actual;
END f_get_maximo_historico_agua;


create or replace FUNCTION f_get_media_historico_agua(p_embalse_nombre IN VARCHAR2)
RETURN NUMBER IS
  v_result SYS_REFCURSOR;
  v_id NUMBER;
  v_ambito_nombre VARCHAR2(100);
  v_agua_total NUMBER;
  v_fecha DATE;
  v_agua_actual NUMBER;
  v_acc_agua_actual NUMBER := 0;
  v_num_metricas NUMBER := 0;
  v_media_agua_actual NUMBER;
BEGIN
    -- Ejecutamos la funciÃ³n y almacenamos el cursor en la variable 'v_result'
    v_result := f_get_historico_agua(p_embalse_nombre);

    -- Iteramos sobre los resultados del cursor
    LOOP
        FETCH v_result INTO v_id, v_ambito_nombre, v_agua_total, v_fecha, v_agua_actual;
        EXIT WHEN v_result%NOTFOUND;

        -- Vamos acumulando los valores de agua actual
        v_acc_agua_actual := v_acc_agua_actual + v_agua_actual;
        v_num_metricas := v_num_metricas + 1;

    END LOOP;

    -- Cerramos el cursor
    CLOSE v_result;

    -- Calculamos la media de agua actual
    v_media_agua_actual := v_acc_agua_actual / v_num_metricas;

    -- Retornamos la media de agua actual
    RETURN v_media_agua_actual;
END f_get_media_historico_agua;


create or replace FUNCTION f_get_minimo_historico_agua(p_embalse_nombre IN VARCHAR2)
RETURN NUMBER IS
  v_result SYS_REFCURSOR;
  v_id NUMBER;
  v_ambito_nombre VARCHAR2(100);
  v_agua_total NUMBER;
  v_fecha DATE;
  v_agua_actual NUMBER;
  v_minimo_agua_actual NUMBER;
  v_first_fetch BOOLEAN := TRUE;
BEGIN
    -- Ejecutamos la funciÃ³n y almacenamos el cursor en la variable 'v_result'
    v_result := f_get_historico_agua(p_embalse_nombre);

    -- Iteramos sobre los resultados del cursor
    LOOP
        FETCH v_result INTO v_id, v_ambito_nombre, v_agua_total, v_fecha, v_agua_actual;
        EXIT WHEN v_result%NOTFOUND;

        -- Inicializamos 'v_minimo_agua_actual' con el primer valor de 'v_agua_actual'
        IF v_first_fetch THEN
            v_minimo_agua_actual := v_agua_actual;
            v_first_fetch := FALSE;
        END IF;

        -- Si el valor actual de agua es menor al mÃ­nimo actual
        IF v_agua_actual < v_minimo_agua_actual THEN
            v_minimo_agua_actual := v_agua_actual;
        END IF;
    END LOOP;

    -- Cerramos el cursor
    CLOSE v_result;

    -- Retornamos el valor mÃ­nimo de agua actual
    RETURN v_minimo_agua_actual;
END f_get_minimo_historico_agua;


-- INDEXES


CREATE UNIQUE INDEX "BACKO"."EMBALSE_PK" ON "BACKO"."EMBALSE" ("ID") 
PCTFREE 10 INITRANS 20 MAXTRANS 255 COMPUTE STATISTICS 
STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 MAXEXTENTS 2147483645
PCTINCREASE 0 FREELISTS 1 FREELIST GROUPS 1
BUFFER_POOL DEFAULT FLASH_CACHE DEFAULT CELL_FLASH_CACHE DEFAULT)
TABLESPACE "DATA" ;


CREATE UNIQUE INDEX "BACKO"."LISTADO_PK" ON "BACKO"."LISTADO" ("CODIGO") 
PCTFREE 10 INITRANS 20 MAXTRANS 255 COMPUTE STATISTICS 
STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 MAXEXTENTS 2147483645
PCTINCREASE 0 FREELISTS 1 FREELIST GROUPS 1
BUFFER_POOL DEFAULT FLASH_CACHE DEFAULT CELL_FLASH_CACHE DEFAULT)
TABLESPACE "DATA" ;

ALTER TABLE backo.agua
ADD CONSTRAINT fk_agua_embalse
FOREIGN KEY (id)
REFERENCES embalse(id);

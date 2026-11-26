use rusqlite::{Error, ffi::SQLITE_CONSTRAINT_FOREIGNKEY};
use sircle::sqlite::{abrir, migrar};

#[test]
fn aplica_la_migracion_una_vez_y_enlaza_una_version_con_su_fuente() {
    let mut db = abrir(":memory:").unwrap();
    let version = |db: &rusqlite::Connection| {
        db.pragma_query_value(None, "user_version", |fila| fila.get::<_, i64>(0))
            .unwrap()
    };
    assert_eq!(version(&db), 0);
    migrar(&mut db).unwrap();
    let preparada = version(&db);
    assert!(preparada >= 1);
    assert_eq!(
        db.pragma_query_value(None, "foreign_keys", |fila| fila.get::<_, i64>(0))
            .unwrap(),
        1
    );

    db.execute_batch(
        "INSERT INTO escenarios VALUES ('P01', 'LR'), ('P02', 'Nannu');
         INSERT INTO fuentes VALUES (10, 'P01', 'sp_actor', 'caso.md');
         INSERT INTO revisiones_fuente VALUES (20, 10, printf('%064d', 1), X'4c52');
         INSERT INTO versiones_clinicas (id, escenario_id, version, estado)
           VALUES (30, 'P01', 'v1', 'pendiente'), (31, 'P02', 'v1', 'pendiente');
         INSERT INTO versiones_fuentes VALUES ('P01', 30, 10, 20);",
    )
    .unwrap();
    migrar(&mut db).unwrap();
    assert_eq!(version(&db), preparada);

    let cadena: (String, i64, i64, i64) = db
        .query_row(
            "SELECT e.id, f.id, r.id, v.id
             FROM escenarios e JOIN fuentes f ON f.escenario_id = e.id
             JOIN revisiones_fuente r ON r.fuente_id = f.id
             JOIN versiones_fuentes vf ON vf.revision_fuente_id = r.id
             JOIN versiones_clinicas v ON v.id = vf.version_clinica_id",
            [],
            |fila| Ok((fila.get(0)?, fila.get(1)?, fila.get(2)?, fila.get(3)?)),
        )
        .unwrap();
    assert_eq!(cadena, ("P01".into(), 10, 20, 30));
    println!("user_version 0 → {preparada} → {preparada} · foreign_keys=1 · cadena={cadena:?}");

    for sql in [
        "INSERT INTO revisiones_fuente VALUES (21, 999, printf('%064d', 1), X'4c52');",
        "INSERT INTO versiones_fuentes VALUES ('P02', 31, 10, 20);",
    ] {
        let error = db.execute(sql, []).unwrap_err();
        assert!(matches!(error, Error::SqliteFailure(causa, _) if causa.extended_code == SQLITE_CONSTRAINT_FOREIGNKEY));
        println!("{sql} → {error}");
    }
}

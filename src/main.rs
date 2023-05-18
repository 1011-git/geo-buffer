use std::io::Write;
use geo::{Polygon, LineString, MultiPolygon};
use geo_svg::{Color, ToSvg, Svg};
use geo_offset::Offset;
use polygon_offset::{skel};

fn create_svg_file(content: &Svg, filename: &mut String){
    filename.push_str(".svg");
    let mut w_file = std::fs::File::create(filename).expect("Cannot create the file");
    w_file.write_all(format!("{content}").to_string().as_bytes()).expect("Cannot write to file");
}

fn make_svg_color<'a>(mp: &'a MultiPolygon, color_name: &'static str) -> Svg<'a> {
    let color = Color::Named(color_name);
    mp.to_svg().with_fill_color(color).with_stroke_width(0.01).with_stroke_color(Color::Named("black"))
}

fn from_wkt(wkt_str: &str) -> Polygon {
    let p1:Polygon = wkt::TryFromWkt::try_from_wkt_str(wkt_str).unwrap();
    p1
}

fn main(){
    println!("Hello! World!");

    let v1 = vec![(0., 0.), (26., 0.), (26., 27.), (0., 27.), (0., 22.), (21., 22.), (21., 5.), (5., 5.), (5., 11.), (10., 11.), (10., 6.), (15., 6.), (15., 11.), (20., 11.), (20., 16.), (15., 16.), (15., 21.), (10., 21.), (10., 16.), (0., 16.)];
    let v1 = vec![(0., 0.), (2., 0.), (2., 2.), (0., 2.)];
    let v2 = vec![(7., 5.3), (8., 5.3), (9.9, 9.), (5.2, 9.)];
    let v2 = vec![(3., 3.), (5., 3.), (5., 5.), (3., 5.)];
    let v3 = vec![(0., 0.), (10., 0.), (8., 7.), (8., 2.), (2., 2.), (2., 8.), (7., 8.), (0., 10.)];
    let p1 = Polygon::new(
        LineString::from(v1), vec![],
    );
    let p2 = Polygon::new(
        LineString::from(v2), vec![],
    );
    let p3 = Polygon::new(
        LineString::from(v3), vec![],
    );
    let p3 = from_wkt("POLYGON((194697.5231090936 550453.0219527445,194617.12731471055 550418.6089381851,194620.46493750607 550378.2733211183,194620.71502714916 550372.5833715772,194631.03491875264 550374.123525286,194640.56484816896 550373.7136821761,194656.58495459537 550359.0540599171,194666.915080847 550346.0243326747,194678.15503572594 550343.1745370318,194710.66504276954 550326.3951977497,194726.95485235925 550330.0254305989,194746.37465303723 550332.6357221996,194759.93462474275 550327.5759819734,194762.504638409 550325.4360408436,194857.93090231204 550385.2260906586,194858.49582112307 550389.9810609655,194857.0838219551 550390.6390328427,194836.8247842316 550403.162604213,194816.2547605363 550414.9721763968,194788.98513542 550405.4118149892,194743.02502397943 550435.4308294704,194726.71224457337 550429.9396114313,194722.15907333975 550442.8564326619,194702.41328437856 550439.6841403831,194701.0532245063 550444.0840825646,194697.5231090936 550453.0219527445))");
    let p3: Polygon = from_wkt("POLYGON((195299.85515485273 550023.4081629915,195293.03545272545 550008.3471758855,195291.06549009934 550007.0171549798,195270.71554891733 550013.5867734824,195267.9557191706 550004.4068038535,195271.45571227657 550003.0768710991,195270.01586959284 549994.0369215967,195264.7659430135 549992.1168526497,195259.79606266745 549987.1868127601,195249.94621858984 549982.4566925989,195243.65640645186 549973.9566605426,195238.36654093818 549968.2666216904,195234.03654794634 549970.0065377305,195231.1965784997 549969.5364957969,195227.87652706023 549974.3964026762,195230.76647096462 549976.4264326956,195222.91637691046 549986.2062264605,195217.80634092315 549991.0061049808,195209.5966160216 549978.0560782707,195201.59689199456 549964.9460562548,195194.1772853983 549944.2561053841,195195.80732988828 549940.6761608367,195196.41733558688 549940.016176047,195214.18724253983 549936.8664880685,195216.93724074855 549935.5965427356,195217.74724385756 549934.9965606804,195219.27727337202 549932.3966065273,195219.73734199163 549927.9066505489,195223.27746238155 549918.6567830064,195225.29750903338 549914.7468474337,195224.49758060093 549910.7068674788,195224.59774171616 549900.6569510249,195225.09775055942 549899.856965606,195231.01781221564 549893.0571164343,195240.01789197183 549883.5873386584,195246.6180006504 549873.5275270196,195249.4581039897 549865.6876366956,195251.4281441199 549862.2076968105,195252.69815962104 549860.6077303184,195254.3781643948 549859.4677666811,195256.0681621612 549858.757799703,195259.79814213098 549858.1278649392,195264.75810376866 549858.017945759,195266.53809516644 549857.6579773775,195268.21809220768 549856.9980098292,195271.1881039453 549854.7780757819,195272.9081394927 549851.7081285233,195273.84821115714 549846.7881837776,195277.68815640282 549848.2582336704,195277.97818225273 549846.5082526112,195282.92813495276 549846.9583287033,195283.69831932336 549835.1284375481,195285.5583010535 549835.3284658899,195285.37811625347 549846.8883687516,195290.6580495277 549848.3784416853,195291.95801645194 549849.7784512206,195300.197930768 549850.9585743761,195304.6280394509 549841.9887188852,195310.8179587542 549843.8888031394,195308.8179285335 549846.7687474345,195307.467885965 549850.0886986168,195316.007777392 549852.5388162577,195319.227763262 549851.7988741739,195320.0178121345 549848.3689148636,195320.00784524652 549846.3189314175,195328.5777314385 549849.0790470131,195331.50768485237 549850.4990826511,195336.16757174267 549855.1791195935,195341.13741149267 549862.6291389485,195343.84730287988 549868.0091387626,195351.27679708216 549895.6690330198,195359.0664465968 549913.5090131186,195365.646116531 549930.6889791004,195371.56574781376 549950.5989121915,195393.64480923247 549997.7588835548,195391.33476314388 550001.7788135542,195320.46481406037 550034.2074070713,195312.56494067278 550030.317311481,195310.63502429082 550026.097314784,195307.50510810938 550022.4672939396,195300.66511947283 550025.1971614566,195299.85515485273 550023.4081629915))");
    //let p3: Polygon = from_wkt("POLYGON((195287.4680356613 550161.1418403401,195293.24301286554 550159.6559455255,195297.53315120962 550148.9161022361,195304.14309754886 550148.92620868,195304.65311340423 550147.6862270084,195310.74302511977 550150.1063054237,195313.98306754153 550145.8463923724,195317.1930325408 550146.4064395418,195317.54306483606 550144.2264629535,195326.33297693884 550145.266596132,195325.5129278192 550148.7265547032,195333.35279861957 550152.8066477835,195335.1127651998 550153.9966664461,195343.01258137313 550161.4367330974,195355.17241640453 550165.5668953928,195372.2721371674 550174.3070997136,195376.30226231704 550164.5172444927,195385.51207771563 550171.3473372301,195401.33189472902 550174.7575643928,195408.64188601053 550171.6277077249,195413.27184257886 550171.9977793277,195413.2018336352 550172.5877733909,195424.16171383677 550174.5179342947,195444.32147171904 550179.4182192556,195443.5914383061 550181.8581875938,195446.12140268344 550182.7982207055,195446.21137358982 550184.5582078071,195452.92126916783 550187.6682905981,195452.40125661556 550188.7082737361,195456.23118903773 550190.9783169553,195457.56119471387 550189.9583467066,195458.6412207829 550187.7983817258,195465.6111181235 550190.6684706633,195478.28099786298 550191.7686658993,195472.86095308812 550197.2685336997,195481.1628335255 550200.5186410074,195485.8208021144 550200.1287192636,195491.2307604269 550199.998807522,195498.09068138423 550201.4589061806,195500.30057260513 550207.0988958162,195500.1705491615 550208.6188813262,195491.26056695593 550211.9887102429,195494.3104821591 550215.7187289858,195498.28049160962 550213.1388140114,195502.94027742566 550224.0887998384,195475.26030691603 550236.1582552895,195486.86977056132 550263.6082185933,195495.53955534834 550272.6082849491,195505.4194934371 550271.4884533266,195525.69946710812 550262.9388499171,195525.63943374963 550265.0388318277,195523.6493960215 550268.3787725201,195529.54930346864 550271.1588449492,195542.11926572517 550267.189079927,195541.78931723023 550264.159099313,195570.18929966036 550250.9896644559,195572.74923558783 550253.6796837862,195582.64911888583 550255.9498248529,195591.95798941597 550259.3089475143,195596.81885819405 550265.0099793845,195600.86865779856 550275.4099598748,195598.8686539979 550276.649917523,195599.48857452086 550281.2698898514,195591.41862903052 550281.939754309,195591.48863716345 550281.399759839,195574.08861100278 550291.7593949011,195576.3985768352 550292.7194243087,195573.88846321404 550301.029316097,195565.7285585861 550299.2091994047,195564.32846047825 550305.9991214782,195554.0485675281 550304.5189678421,195553.6484758352 550310.4089133707,195549.00851258312 550310.4588381707,195540.50855897047 550311.8486898281,195539.79857809457 550311.0186851531,195530.84869759137 550308.0985647006,195517.82890691905 550301.6484074336,195498.02911809593 550298.4881140601,195491.6792131659 550295.7780338034,195485.2693224172 550292.2179595139,195472.8395795123 550282.5078383446,195466.6296466954 550281.4577468153,195452.56996194023 550268.9576221299,195451.52014271895 550258.2676923729,195448.01024445315 550253.7176729008,195434.22047241958 550246.4975095121,195430.54050204076 550246.5074501232,195429.3505089347 550246.6774295532,195426.45039687064 550255.0873142377,195422.47033996048 550260.6172049977,195419.0104164752 550257.6071737782,195412.20048931823 550256.50707299,195409.20051859427 550256.1970271666,195404.1406132148 550252.866972765,195401.91068686036 550249.4169649556,195394.92070878856 550251.5668347678,195386.7408033579 550249.8067172822,195382.7107850488 550252.9666265597,195376.40082274296 550253.7965180954,195373.89085498577 550253.0564836748,195372.1408295071 550255.5164354128,195370.43086123452 550254.406416906,195369.19084129692 550256.2663817517,195365.0709010722 550254.6263287227,195360.27097940713 550252.1762713396,195354.71104680654 550250.786193067,195352.8110836194 550249.4561732921,195349.97110354394 550249.646125969,195348.25107911124 550252.0260788421,195343.73104994767 550256.1059727203,195337.69118344 550250.8559181913,195338.37118679803 550250.3059336342,195336.83122311445 550248.8259208826,195335.49124411354 550248.1959044263,195332.11123650105 550250.3658322559,195323.58147297904 550239.9757795102,195319.58146619605 550242.4056952297,195313.06167608837 550232.6556696605,195311.49171780254 550230.8556590349,195308.5919422259 550218.3857139859,195308.16199678983 550215.2157329075,195307.7923104733 550195.9358841653,195304.3525386181 550183.5059300903,195302.49257043147 550182.4659085916,195300.90269787345 550175.3559409461,195294.82280251686 550171.9158710146,195293.4128235969 550171.3158531863,195290.46185853728 550170.6298112208,195287.55102493233 550161.7658365907,195287.4680356613 550161.1418403401))");
    let mp1 = MultiPolygon::new(vec![p1, p2]);
    let mp2 = polygon_offset::offset_multi_polygon(&mp1, 0.9);
    // let mp3 = p3.offset(4.8).unwrap();
    // let vep = vec![p3.clone()].into();
    let p1 = Polygon::new(
        LineString::from(vec![(0., 0.), (4., 0.), (4., 4.), (2., 1.), (0., 4.)]),
        vec![],
    );
    let p2: MultiPolygon = polygon_offset::offset_polygon(&p1, -0.45);
    let p1: MultiPolygon= vec![p1].into();
    let mut s1 = make_svg_color(&mp2, "orange");
    let mut s2 = make_svg_color(&p1, "red");
    s2 = s2.and(make_svg_color(&p2, "orange"));
    //s1 = s1.and(make_svg_color(&mp1, "orange"));
    //let _binding = skel(&mp1, 1.2);
    //create_svg_file(&s1, &mut String::from("s1"));
    s1 = s1.and(make_svg_color(&mp1, "red"));
    //s1 = s1.and(_binding.to_svg().with_stroke_color(Color::Named("purple")).with_stroke_width(0.2));
    create_svg_file(&s1, &mut String::from("ex3"));
    create_svg_file(&s2, &mut String::from("ex2"));
}
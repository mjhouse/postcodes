use isocountry::CountryCode;
use crate::CodeReference;
const CODES: [CodeReference;139] = [
	( CountryCode::IRL, "F12", "Knock", "Connacht", "C", "", "", "", "", Some(53.7939), Some(-8.9196), Some(4) ),
	( CountryCode::IRL, "F23", "Castlebar", "Connacht", "C", "", "", "", "", Some(53.85), Some(-9.30), Some(4) ),
	( CountryCode::IRL, "F26", "Ballina", "Connacht", "C", "", "", "", "", Some(54.1167), Some(-9.1667), Some(4) ),
	( CountryCode::IRL, "F28", "Westport", "Connacht", "C", "", "", "", "", Some(53.80), Some(-9.5167), Some(4) ),
	( CountryCode::IRL, "F31", "Ballinrobe", "Connacht", "C", "", "", "", "", Some(53.6333), Some(-9.2333), Some(4) ),
	( CountryCode::IRL, "F35", "Ballyhaunis", "Connacht", "C", "", "", "", "", Some(53.7667), Some(-8.7667), Some(4) ),
	( CountryCode::IRL, "F42", "Roscommon", "Connacht", "C", "", "", "", "", Some(53.6333), Some(-8.1833), Some(4) ),
	( CountryCode::IRL, "F45", "Castlerea", "Connacht", "C", "", "", "", "", Some(53.7667), Some(-8.50), Some(4) ),
	( CountryCode::IRL, "F52", "Boyle", "Connacht", "C", "", "", "", "", Some(53.9667), Some(-8.30), Some(4) ),
	( CountryCode::IRL, "F56", "Ballymote", "Connacht", "C", "", "", "", "", Some(54.0833), Some(-8.5167), Some(4) ),
	( CountryCode::IRL, "F91", "Sligo", "Connacht", "C", "", "", "", "", Some(54.2697), Some(-8.4694), Some(4) ),
	( CountryCode::IRL, "H53", "Ballinasloe", "Connacht", "C", "", "", "", "", Some(53.3275), Some(-8.2194), Some(4) ),
	( CountryCode::IRL, "H54", "Tuam", "Connacht", "C", "", "", "", "", Some(53.5167), Some(-8.85), Some(4) ),
	( CountryCode::IRL, "H62", "Loughrea", "Connacht", "C", "", "", "", "", Some(53.1969), Some(-8.5669), Some(4) ),
	( CountryCode::IRL, "H65", "Athenry", "Connacht", "C", "", "", "", "", Some(53.2964), Some(-8.7431), Some(4) ),
	( CountryCode::IRL, "H71", "Clifden", "Connacht", "C", "", "", "", "", Some(53.4891), Some(-10.0191), Some(4) ),
	( CountryCode::IRL, "H91", "Galway", "Connacht", "C", "", "", "", "", Some(53.2719), Some(-9.0489), Some(4) ),
	( CountryCode::IRL, "N41", "Carrick On Shannon", "Connacht", "C", "", "", "", "", Some(53.9469), Some(-8.09), Some(4) ),
	( CountryCode::IRL, "A41", "Ballyboughal", "Leinster", "L", "", "", "", "", Some(53.5195), Some(-6.2667), Some(4) ),
	( CountryCode::IRL, "A42", "Garristown", "Leinster", "L", "", "", "", "", Some(53.5661), Some(-6.3856), Some(4) ),
	( CountryCode::IRL, "A45", "Oldtown", "Leinster", "L", "", "", "", "", Some(53.5253), Some(-6.3156), Some(4) ),
	( CountryCode::IRL, "A63", "Greystones", "Leinster", "L", "", "", "", "", Some(53.1408), Some(-6.0631), Some(4) ),
	( CountryCode::IRL, "A67", "Wicklow", "Leinster", "L", "", "", "", "", Some(52.975), Some(-6.0494), Some(4) ),
	( CountryCode::IRL, "A82", "Kells", "Leinster", "L", "", "", "", "", Some(53.7264), Some(-6.8792), Some(4) ),
	( CountryCode::IRL, "A83", "Enfield", "Leinster", "L", "", "", "", "", Some(53.4142), Some(-6.8323), Some(4) ),
	( CountryCode::IRL, "A84", "Ashbourne", "Leinster", "L", "", "", "", "", Some(53.5116), Some(-6.3982), Some(4) ),
	( CountryCode::IRL, "A85", "Dunshaughlin", "Leinster", "L", "", "", "", "", Some(53.5125), Some(-6.54), Some(4) ),
	( CountryCode::IRL, "A86", "Dunboyne", "Leinster", "L", "", "", "", "", Some(53.40), Some(-6.4667), Some(4) ),
	( CountryCode::IRL, "A91", "Dundalk", "Leinster", "L", "", "", "", "", Some(54.00), Some(-6.4167), Some(4) ),
	( CountryCode::IRL, "A92", "Drogheda", "Leinster", "L", "", "", "", "", Some(53.7189), Some(-6.3478), Some(4) ),
	( CountryCode::IRL, "A94", "Blackrock", "Leinster", "L", "", "", "", "", Some(53.3015), Some(-6.1778), Some(4) ),
	( CountryCode::IRL, "A96", "Dun Laoghaire", "Leinster", "L", "", "", "", "", Some(53.294), Some(-6.1359), Some(4) ),
	( CountryCode::IRL, "A98", "Bray", "Leinster", "L", "", "", "", "", Some(53.2028), Some(-6.0983), Some(4) ),
	( CountryCode::IRL, "C15", "Trim", "Leinster", "L", "", "", "", "", Some(53.555), Some(-6.7917), Some(4) ),
	( CountryCode::IRL, "D01", "Dublin 1", "Leinster", "L", "", "", "", "", Some(53.354), Some(-6.2545), None ),
	( CountryCode::IRL, "D02", "Dublin 2", "Leinster", "L", "", "", "", "", Some(53.34), Some(-6.2543), None ),
	( CountryCode::IRL, "D03", "Dublin 3", "Leinster", "L", "", "", "", "", Some(53.3645), Some(-6.2378), None ),
	( CountryCode::IRL, "D04", "Dublin 4", "Leinster", "L", "", "", "", "", Some(53.3334), Some(-6.2335), None ),
	( CountryCode::IRL, "D05", "Dublin 5", "Leinster", "L", "", "", "", "", Some(53.3842), Some(-6.1921), None ),
	( CountryCode::IRL, "D06", "Dublin 6", "Leinster", "L", "", "", "", "", Some(53.3088), Some(-6.2631), None ),
	( CountryCode::IRL, "D07", "Dublin 7", "Leinster", "L", "", "", "", "", Some(53.3615), Some(-6.2918), None ),
	( CountryCode::IRL, "D08", "Dublin 8", "Leinster", "L", "", "", "", "", Some(53.3346), Some(-6.2733), None ),
	( CountryCode::IRL, "D09", "Dublin 9", "Leinster", "L", "", "", "", "", Some(53.3818), Some(-6.2465), None ),
	( CountryCode::IRL, "D10", "Dublin 10", "Leinster", "L", "", "", "", "", Some(53.3409), Some(-6.3545), None ),
	( CountryCode::IRL, "D11", "Dublin 11", "Leinster", "L", "", "", "", "", Some(53.3899), Some(-6.293), None ),
	( CountryCode::IRL, "D12", "Dublin 12", "Leinster", "L", "", "", "", "", Some(53.322), Some(-6.3165), None ),
	( CountryCode::IRL, "D13", "Dublin 13", "Leinster", "L", "", "", "", "", Some(53.3946), Some(-6.1495), None ),
	( CountryCode::IRL, "D14", "Dublin 14", "Leinster", "L", "", "", "", "", Some(53.296), Some(-6.2593), None ),
	( CountryCode::IRL, "D15", "Dublin 15", "Leinster", "L", "", "", "", "", Some(53.3832), Some(-6.4165), None ),
	( CountryCode::IRL, "D16", "Dublin 16", "Leinster", "L", "", "", "", "", Some(53.3419), Some(-6.279), None ),
	( CountryCode::IRL, "D17", "Dublin 17", "Leinster", "L", "", "", "", "", Some(53.4006), Some(-6.2058), None ),
	( CountryCode::IRL, "D18", "Dublin 18", "Leinster", "L", "", "", "", "", Some(53.2469), Some(-6.1774), None ),
	( CountryCode::IRL, "D20", "Dublin 20", "Leinster", "L", "", "", "", "", Some(53.3518), Some(-6.3693), None ),
	( CountryCode::IRL, "D22", "Dublin 22", "Leinster", "L", "", "", "", "", Some(53.3275), Some(-6.4006), None ),
	( CountryCode::IRL, "D24", "Dublin 24", "Leinster", "L", "", "", "", "", Some(53.2851), Some(-6.3713), None ),
	( CountryCode::IRL, "D6W", "Dublin 6W", "Leinster", "L", "", "", "", "", Some(53.3087), Some(-6.3012), None ),
	( CountryCode::IRL, "K32", "Balbriggan", "Leinster", "L", "", "", "", "", Some(53.6128), Some(-6.1819), Some(4) ),
	( CountryCode::IRL, "K34", "Skerries", "Leinster", "L", "", "", "", "", Some(53.5828), Some(-6.1083), Some(4) ),
	( CountryCode::IRL, "K36", "Malahide", "Leinster", "L", "", "", "", "", Some(53.4508), Some(-6.1544), Some(4) ),
	( CountryCode::IRL, "K45", "Lusk", "Leinster", "L", "", "", "", "", Some(53.5274), Some(-6.1642), Some(4) ),
	( CountryCode::IRL, "K56", "Rush", "Leinster", "L", "", "", "", "", Some(53.5242), Some(-6.105), Some(4) ),
	( CountryCode::IRL, "K67", "Swords", "Leinster", "L", "", "", "", "", Some(53.4597), Some(-6.2181), Some(4) ),
	( CountryCode::IRL, "K78", "Lucan", "Leinster", "L", "", "", "", "", Some(53.3574), Some(-6.4486), Some(4) ),
	( CountryCode::IRL, "N37", "Athlone", "Leinster", "L", "", "", "", "", Some(53.4228), Some(-7.9372), Some(4) ),
	( CountryCode::IRL, "N39", "Longford", "Leinster", "L", "", "", "", "", Some(53.7254), Some(-7.7982), Some(4) ),
	( CountryCode::IRL, "N91", "Mullingar", "Leinster", "L", "", "", "", "", Some(53.5333), Some(-7.35), Some(4) ),
	( CountryCode::IRL, "P14", "Crookstown", "Leinster", "L", "", "", "", "", Some(53.0153), Some(-6.8106), Some(4) ),
	( CountryCode::IRL, "R14", "Athy", "Leinster", "L", "", "", "", "", Some(52.9914), Some(-6.9803), Some(4) ),
	( CountryCode::IRL, "R21", "Bagenalstown", "Leinster", "L", "", "", "", "", Some(52.7003), Some(-6.9618), Some(4) ),
	( CountryCode::IRL, "R32", "Portlaoise", "Leinster", "L", "", "", "", "", Some(53.0344), Some(-7.2998), Some(4) ),
	( CountryCode::IRL, "R35", "Tullamore", "Leinster", "L", "", "", "", "", Some(53.2739), Some(-7.4889), Some(4) ),
	( CountryCode::IRL, "R42", "Birr", "Leinster", "L", "", "", "", "", Some(53.0914), Some(-7.9133), Some(4) ),
	( CountryCode::IRL, "R45", "Edenderry", "Leinster", "L", "", "", "", "", Some(53.3453), Some(-7.0497), Some(4) ),
	( CountryCode::IRL, "R51", "Kildare", "Leinster", "L", "", "", "", "", Some(53.1561), Some(-6.9144), Some(4) ),
	( CountryCode::IRL, "R56", "Curragh", "Leinster", "L", "", "", "", "", Some(53.1469), Some(-6.8304), Some(4) ),
	( CountryCode::IRL, "R93", "Carlow", "Leinster", "L", "", "", "", "", Some(52.8408), Some(-6.9261), Some(4) ),
	( CountryCode::IRL, "R95", "Kilkenny", "Leinster", "L", "", "", "", "", Some(52.6542), Some(-7.2522), Some(4) ),
	( CountryCode::IRL, "W12", "Newbridge", "Leinster", "L", "", "", "", "", Some(53.1819), Some(-6.7967), Some(4) ),
	( CountryCode::IRL, "W23", "Maynooth", "Leinster", "L", "", "", "", "", Some(53.385), Some(-6.5936), Some(4) ),
	( CountryCode::IRL, "W34", "Monasterevin", "Leinster", "L", "", "", "", "", Some(53.1406), Some(-7.0664), Some(4) ),
	( CountryCode::IRL, "W91", "Naas", "Leinster", "L", "", "", "", "", Some(53.2158), Some(-6.6669), Some(4) ),
	( CountryCode::IRL, "Y14", "Arklow", "Leinster", "L", "", "", "", "", Some(52.7931), Some(-6.1414), Some(4) ),
	( CountryCode::IRL, "Y21", "Enniscorthy", "Leinster", "L", "", "", "", "", Some(52.5008), Some(-6.5578), Some(4) ),
	( CountryCode::IRL, "Y25", "Gorey", "Leinster", "L", "", "", "", "", Some(52.6747), Some(-6.2925), Some(4) ),
	( CountryCode::IRL, "Y34", "New Ross", "Leinster", "L", "", "", "", "", Some(52.3967), Some(-6.9367), Some(4) ),
	( CountryCode::IRL, "Y35", "Wexford", "Leinster", "L", "", "", "", "", Some(52.3342), Some(-6.4575), Some(4) ),
	( CountryCode::IRL, "E21", "Cahir", "Munster", "M", "", "", "", "", Some(52.3769), Some(-7.9217), Some(4) ),
	( CountryCode::IRL, "E25", "Cashel", "Munster", "M", "", "", "", "", Some(52.5158), Some(-7.8856), Some(4) ),
	( CountryCode::IRL, "E32", "Carrick on Suir", "Munster", "M", "", "", "", "", Some(52.3492), Some(-7.4131), Some(4) ),
	( CountryCode::IRL, "E34", "Tipperary", "Munster", "M", "", "", "", "", Some(52.4733), Some(-8.1558), Some(4) ),
	( CountryCode::IRL, "E41", "Thurles", "Munster", "M", "", "", "", "", Some(52.6819), Some(-7.8022), Some(4) ),
	( CountryCode::IRL, "E45", "Nenagh", "Munster", "M", "", "", "", "", Some(52.8619), Some(-8.1967), Some(4) ),
	( CountryCode::IRL, "E53", "Roscrea", "Munster", "M", "", "", "", "", Some(52.9511), Some(-7.8017), Some(4) ),
	( CountryCode::IRL, "E91", "Clonmel", "Munster", "M", "", "", "", "", Some(52.355), Some(-7.7039), Some(4) ),
	( CountryCode::IRL, "P12", "Macroom", "Munster", "M", "", "", "", "", Some(51.90), Some(-8.95), Some(4) ),
	( CountryCode::IRL, "P17", "Kinsale", "Munster", "M", "", "", "", "", Some(51.7075), Some(-8.5306), Some(4) ),
	( CountryCode::IRL, "P24", "Cobh", "Munster", "M", "", "", "", "", Some(51.8572), Some(-8.2992), Some(4) ),
	( CountryCode::IRL, "P25", "Midleton", "Munster", "M", "", "", "", "", Some(51.9153), Some(-8.1805), Some(4) ),
	( CountryCode::IRL, "P31", "Ballincollig", "Munster", "M", "", "", "", "", Some(51.8833), Some(-8.5833), Some(4) ),
	( CountryCode::IRL, "P32", "Rylane", "Munster", "M", "", "", "", "", Some(51.9833), Some(-8.8333), Some(4) ),
	( CountryCode::IRL, "P36", "Youghal", "Munster", "M", "", "", "", "", Some(51.95), Some(-7.8506), Some(4) ),
	( CountryCode::IRL, "P43", "Carrigaline", "Munster", "M", "", "", "", "", Some(51.8117), Some(-8.3986), Some(4) ),
	( CountryCode::IRL, "P47", "Dunmanway", "Munster", "M", "", "", "", "", Some(51.7167), Some(-9.1167), Some(4) ),
	( CountryCode::IRL, "P51", "Mallow", "Munster", "M", "", "", "", "", Some(52.1333), Some(-8.6333), Some(4) ),
	( CountryCode::IRL, "P56", "Charleville", "Munster", "M", "", "", "", "", Some(52.35), Some(-8.6833), Some(4) ),
	( CountryCode::IRL, "P61", "Fermoy", "Munster", "M", "", "", "", "", Some(52.1358), Some(-8.2758), Some(4) ),
	( CountryCode::IRL, "P67", "Mitchelstown", "Munster", "M", "", "", "", "", Some(52.2658), Some(-8.2681), Some(4) ),
	( CountryCode::IRL, "P72", "Bandon", "Munster", "M", "", "", "", "", Some(51.7469), Some(-8.7425), Some(4) ),
	( CountryCode::IRL, "P75", "Bantry", "Munster", "M", "", "", "", "", Some(51.6833), Some(-9.45), Some(4) ),
	( CountryCode::IRL, "P81", "Skibbereen", "Munster", "M", "", "", "", "", Some(51.55), Some(-9.2667), Some(4) ),
	( CountryCode::IRL, "P85", "Clonakilty", "Munster", "M", "", "", "", "", Some(51.6231), Some(-8.8706), Some(4) ),
	( CountryCode::IRL, "T12", "Cork city southside", "Munster", "M", "", "", "", "", Some(51.8728), Some(-8.4902), None ),
	( CountryCode::IRL, "T23", "Cork city northside", "Munster", "M", "", "", "", "", Some(51.8972), Some(-8.4702), None ),
	( CountryCode::IRL, "T34", "Carrignavar", "Munster", "M", "", "", "", "", Some(51.9886), Some(-8.4797), Some(4) ),
	( CountryCode::IRL, "T45", "Glanmire", "Munster", "M", "", "", "", "", Some(51.9158), Some(-8.3997), Some(4) ),
	( CountryCode::IRL, "T56", "Watergrasshill", "Munster", "M", "", "", "", "", Some(52.0114), Some(-8.3442), Some(4) ),
	( CountryCode::IRL, "V14", "Shannon", "Munster", "M", "", "", "", "", Some(52.7039), Some(-8.8642), Some(4) ),
	( CountryCode::IRL, "V15", "Kilrush", "Munster", "M", "", "", "", "", Some(52.6397), Some(-9.4833), Some(4) ),
	( CountryCode::IRL, "V23", "Caherciveen", "Munster", "M", "", "", "", "", Some(51.9486), Some(-10.2222), Some(4) ),
	( CountryCode::IRL, "V31", "Listowel", "Munster", "M", "", "", "", "", Some(52.4464), Some(-9.485), Some(4) ),
	( CountryCode::IRL, "V35", "Kilmallock", "Munster", "M", "", "", "", "", Some(52.40), Some(-8.5772), Some(4) ),
	( CountryCode::IRL, "V42", "Newcastle West", "Munster", "M", "", "", "", "", Some(52.4492), Some(-9.0611), Some(4) ),
	( CountryCode::IRL, "V92", "Tralee", "Munster", "M", "", "", "", "", Some(52.2704), Some(-9.7026), Some(4) ),
	( CountryCode::IRL, "V93", "Kenmare", "Munster", "M", "", "", "", "", Some(51.8833), Some(-9.5833), Some(4) ),
	( CountryCode::IRL, "V94", "Limerick", "Munster", "M", "", "", "", "", Some(52.6647), Some(-8.6231), Some(4) ),
	( CountryCode::IRL, "V95", "Miltown Malbay", "Munster", "M", "", "", "", "", Some(52.8553), Some(-9.3983), Some(4) ),
	( CountryCode::IRL, "X35", "Dungarvan", "Munster", "M", "", "", "", "", Some(52.0881), Some(-7.6253), Some(4) ),
	( CountryCode::IRL, "X42", "Kilmacthomas", "Munster", "M", "", "", "", "", Some(52.2097), Some(-7.4258), Some(4) ),
	( CountryCode::IRL, "X91", "Waterford", "Munster", "M", "", "", "", "", Some(52.2583), Some(-7.1119), Some(4) ),
	( CountryCode::IRL, "A75", "Castleblayney", "Ulster", "U", "", "", "", "", Some(54.1167), Some(-6.7333), Some(4) ),
	( CountryCode::IRL, "A81", "Carrickmacross", "Ulster", "U", "", "", "", "", Some(53.9728), Some(-6.7189), Some(4) ),
	( CountryCode::IRL, "F92", "Letterkenny", "Ulster", "U", "", "", "", "", Some(54.95), Some(-7.7333), Some(4) ),
	( CountryCode::IRL, "F93", "Lifford", "Ulster", "U", "", "", "", "", Some(54.8319), Some(-7.4836), Some(4) ),
	( CountryCode::IRL, "F94", "Bundoran", "Ulster", "U", "", "", "", "", Some(54.4778), Some(-8.2809), Some(4) ),
	( CountryCode::IRL, "H12", "Cavan", "Ulster", "U", "", "", "", "", Some(53.9908), Some(-7.3606), Some(4) ),
	( CountryCode::IRL, "H14", "Belturbet", "Ulster", "U", "", "", "", "", Some(54.10), Some(-7.45), Some(4) ),
	( CountryCode::IRL, "H16", "Cootehill", "Ulster", "U", "", "", "", "", Some(54.0725), Some(-7.0819), Some(4) ),
	( CountryCode::IRL, "H18", "Monaghan", "Ulster", "U", "", "", "", "", Some(54.25), Some(-6.9667), Some(4) ),
	( CountryCode::IRL, "H23", "Clones", "Ulster", "U", "", "", "", "", Some(54.1833), Some(-7.2333), Some(4) ),
];
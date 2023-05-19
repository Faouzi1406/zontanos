# Zontanos

Faouzi Bouchkhachekh

Voorblad

Wat is Zontanos?

Zontanos, ook wel leven in het Grieks, is mijn programmeertaal. Zontanos is een gecompileerde taal, waarvan de compiler is geschreven in Rust. Het doel voor mij, op dit moment, is om Zontanos de basis functionaliteit van elke taal te geven, dus functies, If, Else, While, For statements en recursie. Ik ben dan ook niet van plan om de wat complexere systemen zoals, classes, interfaces, etc. te implementeren. Er moet dan ook niet gedacht worden dat mijn taal binnen drie weken een self-hosted compiler zal hebben. Deze taken kosten erg veel tijd en jammer genoeg is tijd erg schaars deze laatste periode. Mijn doel is dan vooral om de volgende functie op de volgende functie te kunnen implementeren: 

fn factorial(i32 n) {
    if n <= 1 {
        return 1
}    else {
        	return n * factorial(n - 1)
}
}


De stappen voor het schrijven van een compiler voor Zontanos

Een programmeertaal schrijven is natuurlijk erg complex en heeft ook erg veel stappen, denk dan aan het schrijven van de lexer, parser die de tokens van de lexer omzet in een Ast(Abstract syntax tree) en tot slot de compiler, die de ast weer omzet in LLVM-ir(Intermediate Representation), waarna door middel van LLVM en Clang deze IR wordt omgezet in een binary executable.
De lexer 
De lexer is het gedeelte van de compiler die alle karakters omzet in tokens die het simpeler maken voor de parser, om deze tokens om te zetten in een Ast. Het gaat over elke karakter en zet deze dan om in de juiste token. 

Een ‘,’ zal bijvoorbeeld geïnterpreteerd worden als de token Tokens::Comma, je vraagt je dan waarschijnlijk af, waarom? De tokens zijn makkelijker en meer correct om tegen aan de matchen dan de karakters zelf, ze maken het makkelijker om de juiste set van stappen uit te voeren gebaseerd op de reeks van tokens die volgen. 

Als we bijvoorbeeld de reeks karakters “let” vinden tijdens het lexen kunnen we deze direct omzetten in de Let keyword. We kunnen nu dus de assumptie maken dat de volgende set van tokens waarschijnlijk een variabele zal zijn. Het maakt het dus makkelijker voor onze parser om correcte keuzes te maken. Een Let token kan dan bijvoorbeeld niet opgevolgd zijn door een Fn(functie token), als dit wel gebeurd kunnen we de gebruiker vragen om dit te veranderen. Als we dit bijvoorbeeld met een set van karakters zouden doen: ‘l’ ‘e’ ‘t’, kun je je voorstellen dat er erg veel opties zijn waar de parser nu rekening mee moet houden. 

 Het is dus erg belangrijk dat de lexer correct is geïmplementeerd, het moet bijvoorbeeld niet karakters omzetten in de verkeerde tokens. Dus hoe bewijs je dan dat de lexer correct is? Testen, om te bewijzen dat de lexer correct is en dit geld eigenlijk voor elke onderdeel van de compiler, is testen erg belangrijk.


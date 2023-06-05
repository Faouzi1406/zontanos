function linkedlist() {
	value; 
	next;

	return {
		append: (list, value) => {
			if(this.value == null) {
				this.value = value;
			}

			if(this.next != null) {
				return this.append(list.next)
			}
		}
	}
}

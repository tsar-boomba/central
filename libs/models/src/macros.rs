/// First arg is a struct declaration including metadata, visibility, and name, *DO NOT INCLUDE id FIELD*
///
/// Second arg is an identifier for "new" version of struct, without id field
///
/// Make sure serde::Serialize and serde::Deserialize are in scope
#[macro_export]
macro_rules! model {
	(
		$id_type:ty, $t_stamp_type:ty, $table_name:tt, $new_name:ident
		$(#[$meta:meta])*
		$struct_name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_name:ident : $field_type:ty
			),*$(,)+
		}
	) => {
		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		#[cfg_attr(feature = "diesel", derive(Insertable, Queryable, Identifiable))]
		#[serde(rename_all = "camelCase")]
		$(#[$meta])*
		pub struct $struct_name {
			pub id: $id_type,
			pub created_at: $t_stamp_type,
        	pub updated_at: $t_stamp_type,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}

		#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Insertable)]
		#[cfg_attr(feature = "diesel", derive(AsChangeset, Insertable))]
		#[serde(rename_all = "camelCase")]
		#[cfg_attr(feature = "diesel", table_name=$table_name)]
		pub struct $new_name {
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}
	};
	// For models which use the nanoid instead of db generating
	(
		$id_type:ty, $t_stamp_type:ty, $table_name:tt, $new_name:ident, $server_gen_id:tt,
		$(#[$meta:meta])*
		$struct_name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_name:ident : $field_type:ty
			),*$(,)+
		}
	) => {
		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		#[cfg_attr(feature = "diesel", derive(Insertable, Queryable, Identifiable))]
		#[serde(rename_all = "camelCase")]
		$(#[$meta])*
		pub struct $struct_name {
			pub id: $id_type,
			pub created_at: $t_stamp_type,
        	pub updated_at: $t_stamp_type,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}

		#[derive(Debug, Clone, Serialize, Deserialize)]
		#[cfg_attr(feature = "diesel", derive(AsChangeset, Insertable))]
		#[cfg_attr(feature = "diesel", table_name=$table_name)]
		#[serde(rename_all = "camelCase")]
		pub struct $new_name {
			#[serde(skip_deserializing)]
			pub id: $id_type,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}
	}
}

#[macro_export]
macro_rules! child_model {
	(
		$id_type:ty, $t_stamp_type:ty, $table_name:tt, $new_name:ident, $parent:ident,
		$(#[$meta:meta])*
		$struct_name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_name:ident : $field_type:ty
			),*$(,)+
		}
	) => {
		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		#[cfg_attr(feature = "diesel", derive(Identifiable, Associations, Queryable, Insertable))]
		#[cfg_attr(feature = "diesel", belongs_to($parent))]
		#[cfg_attr(feature = "diesel", table_name=$table_name)]
		#[serde(rename_all = "camelCase")]
		$(#[$meta])*
		pub struct $struct_name {
			pub id: $id_type,
			pub created_at: $t_stamp_type,
        	pub updated_at: $t_stamp_type,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}

		#[derive(Debug, Clone, Serialize, Deserialize)]
		#[cfg_attr(feature = "diesel", derive(AsChangeset, Insertable))]
		#[serde(rename_all = "camelCase")]
		#[cfg_attr(feature = "diesel", table_name=$table_name)]
		pub struct $new_name {
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}
	};
	// For models which use the nanoid instead of db generating
	(
		$id_type:ty, $t_stamp_type:ty, $table_name:tt, $new_name:ident, $server_gen_id:tt, $parent:ident,
		$(#[$meta:meta])*
		$struct_name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_name:ident : $field_type:ty
			),*$(,)+
		}
	) => {
		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		#[cfg_attr(feature = "diesel", derive(Identifiable, Associations, Queryable, Insertable))]
		#[cfg_attr(feature = "diesel", belongs_to($parent))]
		#[serde(rename_all = "camelCase")]
		$(#[$meta])*
		pub struct $struct_name {
			pub id: $id_type,
			pub created_at: $t_stamp_type,
        	pub updated_at: $t_stamp_type,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}

		#[derive(Debug, Clone, Serialize, Deserialize)]
		#[cfg_attr(feature = "diesel", derive(Identifiable, AsChangeset, Insertable))]
		#[cfg_attr(feature = "diesel", table_name=$table_name)]
		#[serde(rename_all = "camelCase")]
		pub struct $new_name {
			#[serde(skip_deserializing)]
			pub id: $id_type,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type,
			)*
		}
	}
}
